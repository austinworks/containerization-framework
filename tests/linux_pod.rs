#![cfg(feature = "integration")]

//! A pod of containers sharing one VM, and a volume.

mod support;

use containerization_framework as cfw;
use std::os::fd::AsRawFd;

/// Runs a `/bin/sh` script in `container`, returning its exit code and what
/// it wrote to stdout.
fn sh(pod: &cfw::containerization::container::LinuxPod, container: &str, id: &str, script: &str) -> (i32, String) {
  let (mut read, write) = std::io::pipe().expect("a pipe should be creatable");
  let stdout = write.as_raw_fd();
  let script = script.to_string();

  let process = pod
    .exec_in_container(container, id, move |process| {
      process.arguments = vec!["/bin/sh".into(), "-c".into(), script];
      process.stdout = Some(stdout);
      Ok(())
    })
    .unwrap_or_else(|error| panic!("{id} should exec in {container}: {error}"));

  let code = support::container::finish(&process, &format!("{id} in {container}"));
  drop(write);

  (code, support::container::read_to_end(&mut read))
}

#[test]
fn reads_back_the_longest_nbd_timeout() {
  let vmm = support::store::vmm();
  let volume = cfw::containerization::container::linux_pod::PodVolume {
    name: "remote".to_string(),
    source: cfw::containerization::container::linux_pod::pod_volume::Source::Nbd {
      url: "nbd://localhost:10809/remote".to_string(),
      timeout: Some(std::time::Duration::MAX),
      read_only: false,
    },
    format: "ext4".to_string(),
  };
  let configured = volume.clone();

  let pod =
    cfw::containerization::container::LinuxPod::new("cfw-test-nbd-pod", &vmm, support::vm(), move |configuration| {
      configuration.volumes = vec![configured];
      Ok(())
    })
    .expect("a pod should be made");

  assert_eq!(pod.config().volumes, [volume]);
}

#[test]
fn resumes_a_panic_in_a_containers_configuration_on_the_callers_thread() {
  let directory = tempfile::tempdir().expect("a temporary directory");
  let vmm = support::store::vmm();
  let pod = cfw::containerization::container::LinuxPod::new("cfw-test-panic-pod", &vmm, support::vm(), |_| Ok(()))
    .expect("a pod should be made");
  let rootfs = support::store::unpack(&directory.path().join("panics.ext4"));

  let panicked = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
    pod.add_container("panics", rootfs, |_| panic!("the configuration gave up"))
  }))
  .err()
  .expect("the closure's panic should reach the caller");

  assert_eq!(panicked.downcast_ref::<&str>(), Some(&"the configuration gave up"));
  assert!(pod.list_containers().is_empty());
}

#[test]
fn shares_a_volume_between_its_containers() {
  let directory = tempfile::tempdir().expect("a temporary directory");
  let vmm = support::store::vmm();
  let volume = cfw::containerization::container::linux_pod::PodVolume {
    name: "shared".to_string(),
    source: cfw::containerization::container::linux_pod::pod_volume::Source::Tmpfs { size_bytes: None },
    format: "tmpfs".to_string(),
  };
  let configured = volume.clone();

  let pod =
    cfw::containerization::container::LinuxPod::new("cfw-test-pod", &vmm, support::vm(), move |configuration| {
      configuration.hostname = Some("pod".to_string());
      configuration.volumes = vec![configured];
      Ok(())
    })
    .expect("a pod should be made");

  assert_eq!(pod.id(), "cfw-test-pod");
  assert_eq!(pod.vm(), support::vm());
  assert_eq!(pod.config().volumes, [volume]);
  assert_eq!(pod.config().hostname.as_deref(), Some("pod"));

  for name in ["writer", "reader"] {
    let rootfs = support::store::unpack(&directory.path().join(format!("{name}.ext4")));

    pod
      .add_container(name, rootfs, |configuration| {
        configuration.process.arguments = support::container::KEEPALIVE.map(String::from).to_vec();
        configuration.cpus = support::TEST_CPUS;
        configuration.memory_in_bytes = support::TEST_MEMORY_IN_BYTES;
        configuration
          .mounts
          .push(cfw::containerization::container::Mount::shared_mount(
            "shared",
            "/shared",
            &[],
          ));
        Ok(())
      })
      .unwrap_or_else(|error| panic!("{name} should be added: {error}"));
  }

  pod.create().expect("the pod should create");
  pod
    .start_container("writer")
    .expect("the writer should start");
  pod
    .start_container("reader")
    .expect("the reader should start");

  let mut containers = pod.list_containers();
  containers.sort();
  assert_eq!(containers, ["reader", "writer"]);

  assert_eq!(sh(&pod, "writer", "write", "echo hello > /shared/greeting").0, 0);
  assert_eq!(
    sh(&pod, "reader", "read", "cat /shared/greeting"),
    (0, "hello\n".to_string()),
    "the reader sees what the writer wrote"
  );
  assert_eq!(
    sh(&pod, "reader", "hostname", "hostname"),
    (0, "pod\n".to_string()),
    "a container takes the pod's hostname"
  );

  let statistics = pod
    .statistics(
      Some(&["writer"]),
      cfw::containerization::container::StatCategory::PROCESS,
    )
    .expect("the pod should report statistics");
  let [writer] = statistics.as_slice() else {
    panic!("the pod should report the one container asked for, not {statistics:?}");
  };
  assert_eq!(writer.id, "writer");
  assert!(writer.process.is_some());

  pod
    .with_virtual_machine_instance(|instance| {
      assert_eq!(
        instance.state(),
        cfw::containerization::vm::VirtualMachineInstanceState::Running
      );
      Ok(())
    })
    .expect("the pod's VM should be reachable");

  pod
    .stop_container("reader")
    .expect("the reader should stop");
  pod.stop().expect("the pod should stop");
}
