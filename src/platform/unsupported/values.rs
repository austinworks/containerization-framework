//! Stand-ins for `Values.swift`: what Containerization's value types compute.

use super::CzOutcome;
use super::CzTerminal;
use super::HostsEntryName;
use super::LogLevel;
use crate::containerization::container;
use crate::containerization::network;
use crate::containerization::process;
use crate::containerization_oci;
use crate::platform;

taken!(
  command_line_kernel_args -> Vec<String>,
  command_line_init_args -> Vec<String>,
  hosts_entry -> network::hosts::Entry,
);

failing!(
  cz_kernel_command_line_add_debug(Vec<String>, Vec<String>),
  cz_kernel_command_line_add_panic(Vec<String>, Vec<String>, i64),
  cz_kernel_command_line_set_agent_log_level(Vec<String>, Vec<String>, LogLevel),
  cz_mount_clone(container::Mount, &str),
  cz_mount_tag_hash(container::Mount),
  cz_dns_validate(network::DNS),
  cz_dns_resolv_conf(network::DNS),
  cz_hosts_file(network::Hosts),
  cz_hosts_entry_rendered(network::hosts::Entry),
  cz_hosts_entry_named(HostsEntryName, Option<String>),
  cz_exit_status_new(i32),
  cz_linux_rlimit_kind_parse(&str),
  cz_linux_process_configuration_from_image_config(
    containerization_oci::image::ImageConfig,
    process::LinuxProcessConfiguration,
    platform::ConfigureProcess,
  ),
  cz_linux_process_configuration_set_terminal_io(
    process::LinuxProcessConfiguration,
    CzTerminal,
    platform::ConfigureProcess,
  ),
  cz_signal_parse(&str),
  cz_signal_parse_from(&str, Vec<String>, Vec<i32>),
  cz_signal_linux(),
  cz_signal_platform(),
  cz_signal_platform_name(i32),
  cz_signal_linux_signal(i32),
);
