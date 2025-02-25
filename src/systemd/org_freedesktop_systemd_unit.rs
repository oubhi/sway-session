use dbus as dbus;
#[allow(unused_imports)]
use dbus::arg;
use dbus::blocking;

pub trait OrgFreedesktopDBusPeer {
    fn ping(&self) -> Result<(), dbus::Error>;
    fn get_machine_id(&self) -> Result<String, dbus::Error>;
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target=T>> OrgFreedesktopDBusPeer for blocking::Proxy<'a, C> {

    fn ping(&self) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.DBus.Peer", "Ping", ())
    }

    fn get_machine_id(&self) -> Result<String, dbus::Error> {
        self.method_call("org.freedesktop.DBus.Peer", "GetMachineId", ())
            .and_then(|r: (String, )| Ok(r.0, ))
    }
}

pub trait OrgFreedesktopDBusIntrospectable {
    fn introspect(&self) -> Result<String, dbus::Error>;
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target=T>> OrgFreedesktopDBusIntrospectable for blocking::Proxy<'a, C> {

    fn introspect(&self) -> Result<String, dbus::Error> {
        self.method_call("org.freedesktop.DBus.Introspectable", "Introspect", ())
            .and_then(|r: (String, )| Ok(r.0, ))
    }
}

pub trait OrgFreedesktopDBusProperties {
    fn get<R0: for<'b> arg::Get<'b> + 'static>(&self, interface_name: &str, property_name: &str) -> Result<R0, dbus::Error>;
    fn get_all(&self, interface_name: &str) -> Result<arg::PropMap, dbus::Error>;
    fn set<I2: arg::Arg + arg::Append>(&self, interface_name: &str, property_name: &str, value: I2) -> Result<(), dbus::Error>;
}

#[derive(Debug)]
pub struct OrgFreedesktopDBusPropertiesPropertiesChanged {
    pub interface_name: String,
    pub changed_properties: arg::PropMap,
    pub invalidated_properties: Vec<String>,
}

impl arg::AppendAll for OrgFreedesktopDBusPropertiesPropertiesChanged {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.interface_name, i);
        arg::RefArg::append(&self.changed_properties, i);
        arg::RefArg::append(&self.invalidated_properties, i);
    }
}

impl arg::ReadAll for OrgFreedesktopDBusPropertiesPropertiesChanged {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(OrgFreedesktopDBusPropertiesPropertiesChanged {
            interface_name: i.read()?,
            changed_properties: i.read()?,
            invalidated_properties: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for OrgFreedesktopDBusPropertiesPropertiesChanged {
    const NAME: &'static str = "PropertiesChanged";
    const INTERFACE: &'static str = "org.freedesktop.DBus.Properties";
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target=T>> OrgFreedesktopDBusProperties for blocking::Proxy<'a, C> {

    fn get<R0: for<'b> arg::Get<'b> + 'static>(&self, interface_name: &str, property_name: &str) -> Result<R0, dbus::Error> {
        self.method_call("org.freedesktop.DBus.Properties", "Get", (interface_name, property_name, ))
            .and_then(|r: (arg::Variant<R0>, )| Ok((r.0).0, ))
    }

    fn get_all(&self, interface_name: &str) -> Result<arg::PropMap, dbus::Error> {
        self.method_call("org.freedesktop.DBus.Properties", "GetAll", (interface_name, ))
            .and_then(|r: (arg::PropMap, )| Ok(r.0, ))
    }

    fn set<I2: arg::Arg + arg::Append>(&self, interface_name: &str, property_name: &str, value: I2) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.DBus.Properties", "Set", (interface_name, property_name, arg::Variant(value), ))
    }
}

pub trait OrgFreedesktopSystemd1Service {
    fn bind_mount(&self, source: &str, destination: &str, read_only: bool, mkdir: bool) -> Result<(), dbus::Error>;
    fn mount_image(&self, source: &str, destination: &str, read_only: bool, mkdir: bool, options: Vec<(&str, &str,)>) -> Result<(), dbus::Error>;
    fn dump_file_descriptor_store(&self) -> Result<Vec<(String, u32, u32, u32, u64, u32, u32, String, u32,)>, dbus::Error>;
    fn get_processes(&self) -> Result<Vec<(String, u32, String,)>, dbus::Error>;
    fn attach_processes(&self, subcgroup: &str, pids: Vec<u32>) -> Result<(), dbus::Error>;
    fn type_(&self) -> Result<String, dbus::Error>;
    fn exit_type(&self) -> Result<String, dbus::Error>;
    fn restart(&self) -> Result<String, dbus::Error>;
    fn restart_mode(&self) -> Result<String, dbus::Error>;
    fn pidfile(&self) -> Result<String, dbus::Error>;
    fn notify_access(&self) -> Result<String, dbus::Error>;
    fn restart_usec(&self) -> Result<u64, dbus::Error>;
    fn restart_steps(&self) -> Result<u32, dbus::Error>;
    fn restart_max_delay_usec(&self) -> Result<u64, dbus::Error>;
    fn restart_usec_next(&self) -> Result<u64, dbus::Error>;
    fn timeout_start_usec(&self) -> Result<u64, dbus::Error>;
    fn timeout_stop_usec(&self) -> Result<u64, dbus::Error>;
    fn timeout_abort_usec(&self) -> Result<u64, dbus::Error>;
    fn timeout_start_failure_mode(&self) -> Result<String, dbus::Error>;
    fn timeout_stop_failure_mode(&self) -> Result<String, dbus::Error>;
    fn runtime_max_usec(&self) -> Result<u64, dbus::Error>;
    fn runtime_randomized_extra_usec(&self) -> Result<u64, dbus::Error>;
    fn watchdog_usec(&self) -> Result<u64, dbus::Error>;
    fn watchdog_timestamp(&self) -> Result<u64, dbus::Error>;
    fn watchdog_timestamp_monotonic(&self) -> Result<u64, dbus::Error>;
    fn root_directory_start_only(&self) -> Result<bool, dbus::Error>;
    fn remain_after_exit(&self) -> Result<bool, dbus::Error>;
    fn guess_main_pid(&self) -> Result<bool, dbus::Error>;
    fn restart_prevent_exit_status(&self) -> Result<(Vec<i32>, Vec<i32>,), dbus::Error>;
    fn restart_force_exit_status(&self) -> Result<(Vec<i32>, Vec<i32>,), dbus::Error>;
    fn success_exit_status(&self) -> Result<(Vec<i32>, Vec<i32>,), dbus::Error>;
    fn main_pid(&self) -> Result<u32, dbus::Error>;
    fn control_pid(&self) -> Result<u32, dbus::Error>;
    fn bus_name(&self) -> Result<String, dbus::Error>;
    fn file_descriptor_store_max(&self) -> Result<u32, dbus::Error>;
    fn nfile_descriptor_store(&self) -> Result<u32, dbus::Error>;
    fn file_descriptor_store_preserve(&self) -> Result<String, dbus::Error>;
    fn status_text(&self) -> Result<String, dbus::Error>;
    fn status_errno(&self) -> Result<i32, dbus::Error>;
    fn status_bus_error(&self) -> Result<String, dbus::Error>;
    fn status_varlink_error(&self) -> Result<String, dbus::Error>;
    fn result(&self) -> Result<String, dbus::Error>;
    fn reload_result(&self) -> Result<String, dbus::Error>;
    fn clean_result(&self) -> Result<String, dbus::Error>;
    fn live_mount_result(&self) -> Result<String, dbus::Error>;
    fn usbfunction_descriptors(&self) -> Result<String, dbus::Error>;
    fn usbfunction_strings(&self) -> Result<String, dbus::Error>;
    fn uid(&self) -> Result<u32, dbus::Error>;
    fn gid(&self) -> Result<u32, dbus::Error>;
    fn nrestarts(&self) -> Result<u32, dbus::Error>;
    fn oompolicy(&self) -> Result<String, dbus::Error>;
    fn open_file(&self) -> Result<Vec<(String, String, u64,)>, dbus::Error>;
    fn extra_file_descriptor_names(&self) -> Result<Vec<String>, dbus::Error>;
    fn reload_signal(&self) -> Result<i32, dbus::Error>;
    fn exec_main_start_timestamp(&self) -> Result<u64, dbus::Error>;
    fn exec_main_start_timestamp_monotonic(&self) -> Result<u64, dbus::Error>;
    fn exec_main_exit_timestamp(&self) -> Result<u64, dbus::Error>;
    fn exec_main_exit_timestamp_monotonic(&self) -> Result<u64, dbus::Error>;
    fn exec_main_handoff_timestamp(&self) -> Result<u64, dbus::Error>;
    fn exec_main_handoff_timestamp_monotonic(&self) -> Result<u64, dbus::Error>;
    fn exec_main_pid(&self) -> Result<u32, dbus::Error>;
    fn exec_main_code(&self) -> Result<i32, dbus::Error>;
    fn exec_main_status(&self) -> Result<i32, dbus::Error>;
    fn exec_condition(&self) -> Result<Vec<(String, Vec<String>, bool, u64, u64, u64, u64, u32, i32, i32,)>, dbus::Error>;
    fn exec_condition_ex(&self) -> Result<Vec<(String, Vec<String>, Vec<String>, u64, u64, u64, u64, u32, i32, i32,)>, dbus::Error>;
    fn exec_start_pre(&self) -> Result<Vec<(String, Vec<String>, bool, u64, u64, u64, u64, u32, i32, i32,)>, dbus::Error>;
    fn exec_start_pre_ex(&self) -> Result<Vec<(String, Vec<String>, Vec<String>, u64, u64, u64, u64, u32, i32, i32,)>, dbus::Error>;
    fn exec_start(&self) -> Result<Vec<(String, Vec<String>, bool, u64, u64, u64, u64, u32, i32, i32,)>, dbus::Error>;
    fn exec_start_ex(&self) -> Result<Vec<(String, Vec<String>, Vec<String>, u64, u64, u64, u64, u32, i32, i32,)>, dbus::Error>;
    fn exec_start_post(&self) -> Result<Vec<(String, Vec<String>, bool, u64, u64, u64, u64, u32, i32, i32,)>, dbus::Error>;
    fn exec_start_post_ex(&self) -> Result<Vec<(String, Vec<String>, Vec<String>, u64, u64, u64, u64, u32, i32, i32,)>, dbus::Error>;
    fn exec_reload(&self) -> Result<Vec<(String, Vec<String>, bool, u64, u64, u64, u64, u32, i32, i32,)>, dbus::Error>;
    fn exec_reload_ex(&self) -> Result<Vec<(String, Vec<String>, Vec<String>, u64, u64, u64, u64, u32, i32, i32,)>, dbus::Error>;
    fn exec_stop(&self) -> Result<Vec<(String, Vec<String>, bool, u64, u64, u64, u64, u32, i32, i32,)>, dbus::Error>;
    fn exec_stop_ex(&self) -> Result<Vec<(String, Vec<String>, Vec<String>, u64, u64, u64, u64, u32, i32, i32,)>, dbus::Error>;
    fn exec_stop_post(&self) -> Result<Vec<(String, Vec<String>, bool, u64, u64, u64, u64, u32, i32, i32,)>, dbus::Error>;
    fn exec_stop_post_ex(&self) -> Result<Vec<(String, Vec<String>, Vec<String>, u64, u64, u64, u64, u32, i32, i32,)>, dbus::Error>;
    fn slice(&self) -> Result<String, dbus::Error>;
    fn control_group(&self) -> Result<String, dbus::Error>;
    fn control_group_id(&self) -> Result<u64, dbus::Error>;
    fn memory_current(&self) -> Result<u64, dbus::Error>;
    fn memory_peak(&self) -> Result<u64, dbus::Error>;
    fn memory_swap_current(&self) -> Result<u64, dbus::Error>;
    fn memory_swap_peak(&self) -> Result<u64, dbus::Error>;
    fn memory_zswap_current(&self) -> Result<u64, dbus::Error>;
    fn memory_available(&self) -> Result<u64, dbus::Error>;
    fn effective_memory_max(&self) -> Result<u64, dbus::Error>;
    fn effective_memory_high(&self) -> Result<u64, dbus::Error>;
    fn cpuusage_nsec(&self) -> Result<u64, dbus::Error>;
    fn effective_cpus(&self) -> Result<Vec<u8>, dbus::Error>;
    fn effective_memory_nodes(&self) -> Result<Vec<u8>, dbus::Error>;
    fn tasks_current(&self) -> Result<u64, dbus::Error>;
    fn effective_tasks_max(&self) -> Result<u64, dbus::Error>;
    fn ipingress_bytes(&self) -> Result<u64, dbus::Error>;
    fn ipingress_packets(&self) -> Result<u64, dbus::Error>;
    fn ipegress_bytes(&self) -> Result<u64, dbus::Error>;
    fn ipegress_packets(&self) -> Result<u64, dbus::Error>;
    fn ioread_bytes(&self) -> Result<u64, dbus::Error>;
    fn ioread_operations(&self) -> Result<u64, dbus::Error>;
    fn iowrite_bytes(&self) -> Result<u64, dbus::Error>;
    fn iowrite_operations(&self) -> Result<u64, dbus::Error>;
    fn delegate(&self) -> Result<bool, dbus::Error>;
    fn delegate_controllers(&self) -> Result<Vec<String>, dbus::Error>;
    fn delegate_subgroup(&self) -> Result<String, dbus::Error>;
    fn cpuaccounting(&self) -> Result<bool, dbus::Error>;
    fn cpuweight(&self) -> Result<u64, dbus::Error>;
    fn startup_cpuweight(&self) -> Result<u64, dbus::Error>;
    fn cpushares(&self) -> Result<u64, dbus::Error>;
    fn startup_cpushares(&self) -> Result<u64, dbus::Error>;
    fn cpuquota_per_sec_usec(&self) -> Result<u64, dbus::Error>;
    fn cpuquota_period_usec(&self) -> Result<u64, dbus::Error>;
    fn allowed_cpus(&self) -> Result<Vec<u8>, dbus::Error>;
    fn startup_allowed_cpus(&self) -> Result<Vec<u8>, dbus::Error>;
    fn allowed_memory_nodes(&self) -> Result<Vec<u8>, dbus::Error>;
    fn startup_allowed_memory_nodes(&self) -> Result<Vec<u8>, dbus::Error>;
    fn ioaccounting(&self) -> Result<bool, dbus::Error>;
    fn ioweight(&self) -> Result<u64, dbus::Error>;
    fn startup_ioweight(&self) -> Result<u64, dbus::Error>;
    fn iodevice_weight(&self) -> Result<Vec<(String, u64,)>, dbus::Error>;
    fn ioread_bandwidth_max(&self) -> Result<Vec<(String, u64,)>, dbus::Error>;
    fn iowrite_bandwidth_max(&self) -> Result<Vec<(String, u64,)>, dbus::Error>;
    fn ioread_iopsmax(&self) -> Result<Vec<(String, u64,)>, dbus::Error>;
    fn iowrite_iopsmax(&self) -> Result<Vec<(String, u64,)>, dbus::Error>;
    fn iodevice_latency_target_usec(&self) -> Result<Vec<(String, u64,)>, dbus::Error>;
    fn block_ioaccounting(&self) -> Result<bool, dbus::Error>;
    fn block_ioweight(&self) -> Result<u64, dbus::Error>;
    fn startup_block_ioweight(&self) -> Result<u64, dbus::Error>;
    fn block_iodevice_weight(&self) -> Result<Vec<(String, u64,)>, dbus::Error>;
    fn block_ioread_bandwidth(&self) -> Result<Vec<(String, u64,)>, dbus::Error>;
    fn block_iowrite_bandwidth(&self) -> Result<Vec<(String, u64,)>, dbus::Error>;
    fn memory_accounting(&self) -> Result<bool, dbus::Error>;
    fn default_memory_low(&self) -> Result<u64, dbus::Error>;
    fn default_startup_memory_low(&self) -> Result<u64, dbus::Error>;
    fn default_memory_min(&self) -> Result<u64, dbus::Error>;
    fn memory_min(&self) -> Result<u64, dbus::Error>;
    fn memory_low(&self) -> Result<u64, dbus::Error>;
    fn startup_memory_low(&self) -> Result<u64, dbus::Error>;
    fn memory_high(&self) -> Result<u64, dbus::Error>;
    fn startup_memory_high(&self) -> Result<u64, dbus::Error>;
    fn memory_max(&self) -> Result<u64, dbus::Error>;
    fn startup_memory_max(&self) -> Result<u64, dbus::Error>;
    fn memory_swap_max(&self) -> Result<u64, dbus::Error>;
    fn startup_memory_swap_max(&self) -> Result<u64, dbus::Error>;
    fn memory_zswap_max(&self) -> Result<u64, dbus::Error>;
    fn startup_memory_zswap_max(&self) -> Result<u64, dbus::Error>;
    fn memory_zswap_writeback(&self) -> Result<bool, dbus::Error>;
    fn memory_limit(&self) -> Result<u64, dbus::Error>;
    fn device_policy(&self) -> Result<String, dbus::Error>;
    fn device_allow(&self) -> Result<Vec<(String, String,)>, dbus::Error>;
    fn tasks_accounting(&self) -> Result<bool, dbus::Error>;
    fn tasks_max(&self) -> Result<u64, dbus::Error>;
    fn ipaccounting(&self) -> Result<bool, dbus::Error>;
    fn ipaddress_allow(&self) -> Result<Vec<(i32, Vec<u8>, u32,)>, dbus::Error>;
    fn ipaddress_deny(&self) -> Result<Vec<(i32, Vec<u8>, u32,)>, dbus::Error>;
    fn ipingress_filter_path(&self) -> Result<Vec<String>, dbus::Error>;
    fn ipegress_filter_path(&self) -> Result<Vec<String>, dbus::Error>;
    fn disable_controllers(&self) -> Result<Vec<String>, dbus::Error>;
    fn managed_oomswap(&self) -> Result<String, dbus::Error>;
    fn managed_oommemory_pressure(&self) -> Result<String, dbus::Error>;
    fn managed_oommemory_pressure_limit(&self) -> Result<u32, dbus::Error>;
    fn managed_oommemory_pressure_duration_usec(&self) -> Result<u64, dbus::Error>;
    fn managed_oompreference(&self) -> Result<String, dbus::Error>;
    fn bpfprogram(&self) -> Result<Vec<(String, String,)>, dbus::Error>;
    fn socket_bind_allow(&self) -> Result<Vec<(i32, i32, u16, u16,)>, dbus::Error>;
    fn socket_bind_deny(&self) -> Result<Vec<(i32, i32, u16, u16,)>, dbus::Error>;
    fn restrict_network_interfaces(&self) -> Result<(bool, Vec<String>,), dbus::Error>;
    fn memory_pressure_watch(&self) -> Result<String, dbus::Error>;
    fn memory_pressure_threshold_usec(&self) -> Result<u64, dbus::Error>;
    fn nftset(&self) -> Result<Vec<(i32, i32, String, String,)>, dbus::Error>;
    fn coredump_receive(&self) -> Result<bool, dbus::Error>;
    fn environment(&self) -> Result<Vec<String>, dbus::Error>;
    fn environment_files(&self) -> Result<Vec<(String, bool,)>, dbus::Error>;
    fn pass_environment(&self) -> Result<Vec<String>, dbus::Error>;
    fn unset_environment(&self) -> Result<Vec<String>, dbus::Error>;
    fn umask(&self) -> Result<u32, dbus::Error>;
    fn limit_cpu(&self) -> Result<u64, dbus::Error>;
    fn limit_cpusoft(&self) -> Result<u64, dbus::Error>;
    fn limit_fsize(&self) -> Result<u64, dbus::Error>;
    fn limit_fsizesoft(&self) -> Result<u64, dbus::Error>;
    fn limit_data(&self) -> Result<u64, dbus::Error>;
    fn limit_datasoft(&self) -> Result<u64, dbus::Error>;
    fn limit_stack(&self) -> Result<u64, dbus::Error>;
    fn limit_stacksoft(&self) -> Result<u64, dbus::Error>;
    fn limit_core(&self) -> Result<u64, dbus::Error>;
    fn limit_coresoft(&self) -> Result<u64, dbus::Error>;
    fn limit_rss(&self) -> Result<u64, dbus::Error>;
    fn limit_rsssoft(&self) -> Result<u64, dbus::Error>;
    fn limit_nofile(&self) -> Result<u64, dbus::Error>;
    fn limit_nofilesoft(&self) -> Result<u64, dbus::Error>;
    fn limit_as(&self) -> Result<u64, dbus::Error>;
    fn limit_assoft(&self) -> Result<u64, dbus::Error>;
    fn limit_nproc(&self) -> Result<u64, dbus::Error>;
    fn limit_nprocsoft(&self) -> Result<u64, dbus::Error>;
    fn limit_memlock(&self) -> Result<u64, dbus::Error>;
    fn limit_memlocksoft(&self) -> Result<u64, dbus::Error>;
    fn limit_locks(&self) -> Result<u64, dbus::Error>;
    fn limit_lockssoft(&self) -> Result<u64, dbus::Error>;
    fn limit_sigpending(&self) -> Result<u64, dbus::Error>;
    fn limit_sigpendingsoft(&self) -> Result<u64, dbus::Error>;
    fn limit_msgqueue(&self) -> Result<u64, dbus::Error>;
    fn limit_msgqueuesoft(&self) -> Result<u64, dbus::Error>;
    fn limit_nice(&self) -> Result<u64, dbus::Error>;
    fn limit_nicesoft(&self) -> Result<u64, dbus::Error>;
    fn limit_rtprio(&self) -> Result<u64, dbus::Error>;
    fn limit_rtpriosoft(&self) -> Result<u64, dbus::Error>;
    fn limit_rttime(&self) -> Result<u64, dbus::Error>;
    fn limit_rttimesoft(&self) -> Result<u64, dbus::Error>;
    fn working_directory(&self) -> Result<String, dbus::Error>;
    fn root_directory(&self) -> Result<String, dbus::Error>;
    fn root_image(&self) -> Result<String, dbus::Error>;
    fn root_image_options(&self) -> Result<Vec<(String, String,)>, dbus::Error>;
    fn root_hash(&self) -> Result<Vec<u8>, dbus::Error>;
    fn root_hash_path(&self) -> Result<String, dbus::Error>;
    fn root_hash_signature(&self) -> Result<Vec<u8>, dbus::Error>;
    fn root_hash_signature_path(&self) -> Result<String, dbus::Error>;
    fn root_verity(&self) -> Result<String, dbus::Error>;
    fn root_ephemeral(&self) -> Result<bool, dbus::Error>;
    fn extension_directories(&self) -> Result<Vec<String>, dbus::Error>;
    fn extension_images(&self) -> Result<Vec<(String, bool, Vec<(String, String,)>,)>, dbus::Error>;
    fn mount_images(&self) -> Result<Vec<(String, String, bool, Vec<(String, String,)>,)>, dbus::Error>;
    fn oomscore_adjust(&self) -> Result<i32, dbus::Error>;
    fn coredump_filter(&self) -> Result<u64, dbus::Error>;
    fn nice(&self) -> Result<i32, dbus::Error>;
    fn ioscheduling_class(&self) -> Result<i32, dbus::Error>;
    fn ioscheduling_priority(&self) -> Result<i32, dbus::Error>;
    fn cpuscheduling_policy(&self) -> Result<i32, dbus::Error>;
    fn cpuscheduling_priority(&self) -> Result<i32, dbus::Error>;
    fn cpuaffinity(&self) -> Result<Vec<u8>, dbus::Error>;
    fn cpuaffinity_from_numa(&self) -> Result<bool, dbus::Error>;
    fn numapolicy(&self) -> Result<i32, dbus::Error>;
    fn numamask(&self) -> Result<Vec<u8>, dbus::Error>;
    fn timer_slack_nsec(&self) -> Result<u64, dbus::Error>;
    fn cpuscheduling_reset_on_fork(&self) -> Result<bool, dbus::Error>;
    fn non_blocking(&self) -> Result<bool, dbus::Error>;
    fn standard_input(&self) -> Result<String, dbus::Error>;
    fn standard_input_file_descriptor_name(&self) -> Result<String, dbus::Error>;
    fn standard_input_data(&self) -> Result<Vec<u8>, dbus::Error>;
    fn standard_output(&self) -> Result<String, dbus::Error>;
    fn standard_output_file_descriptor_name(&self) -> Result<String, dbus::Error>;
    fn standard_error(&self) -> Result<String, dbus::Error>;
    fn standard_error_file_descriptor_name(&self) -> Result<String, dbus::Error>;
    fn ttypath(&self) -> Result<String, dbus::Error>;
    fn ttyreset(&self) -> Result<bool, dbus::Error>;
    fn ttyvhangup(&self) -> Result<bool, dbus::Error>;
    fn ttyvtdisallocate(&self) -> Result<bool, dbus::Error>;
    fn ttyrows(&self) -> Result<u16, dbus::Error>;
    fn ttycolumns(&self) -> Result<u16, dbus::Error>;
    fn syslog_priority(&self) -> Result<i32, dbus::Error>;
    fn syslog_identifier(&self) -> Result<String, dbus::Error>;
    fn syslog_level_prefix(&self) -> Result<bool, dbus::Error>;
    fn syslog_level(&self) -> Result<i32, dbus::Error>;
    fn syslog_facility(&self) -> Result<i32, dbus::Error>;
    fn log_level_max(&self) -> Result<i32, dbus::Error>;
    fn log_rate_limit_interval_usec(&self) -> Result<u64, dbus::Error>;
    fn log_rate_limit_burst(&self) -> Result<u32, dbus::Error>;
    fn log_extra_fields(&self) -> Result<Vec<Vec<u8>>, dbus::Error>;
    fn log_filter_patterns(&self) -> Result<Vec<(bool, String,)>, dbus::Error>;
    fn log_namespace(&self) -> Result<String, dbus::Error>;
    fn secure_bits(&self) -> Result<i32, dbus::Error>;
    fn capability_bounding_set(&self) -> Result<u64, dbus::Error>;
    fn ambient_capabilities(&self) -> Result<u64, dbus::Error>;
    fn user(&self) -> Result<String, dbus::Error>;
    fn group(&self) -> Result<String, dbus::Error>;
    fn dynamic_user(&self) -> Result<bool, dbus::Error>;
    fn set_login_environment(&self) -> Result<bool, dbus::Error>;
    fn remove_ipc(&self) -> Result<bool, dbus::Error>;
    fn set_credential(&self) -> Result<Vec<(String, Vec<u8>,)>, dbus::Error>;
    fn set_credential_encrypted(&self) -> Result<Vec<(String, Vec<u8>,)>, dbus::Error>;
    fn load_credential(&self) -> Result<Vec<(String, String,)>, dbus::Error>;
    fn load_credential_encrypted(&self) -> Result<Vec<(String, String,)>, dbus::Error>;
    fn import_credential(&self) -> Result<Vec<String>, dbus::Error>;
    fn import_credential_ex(&self) -> Result<Vec<(String, String,)>, dbus::Error>;
    fn supplementary_groups(&self) -> Result<Vec<String>, dbus::Error>;
    fn pamname(&self) -> Result<String, dbus::Error>;
    fn read_write_paths(&self) -> Result<Vec<String>, dbus::Error>;
    fn read_only_paths(&self) -> Result<Vec<String>, dbus::Error>;
    fn inaccessible_paths(&self) -> Result<Vec<String>, dbus::Error>;
    fn exec_paths(&self) -> Result<Vec<String>, dbus::Error>;
    fn no_exec_paths(&self) -> Result<Vec<String>, dbus::Error>;
    fn exec_search_path(&self) -> Result<Vec<String>, dbus::Error>;
    fn mount_flags(&self) -> Result<u64, dbus::Error>;
    fn private_tmp(&self) -> Result<bool, dbus::Error>;
    fn private_tmp_ex(&self) -> Result<String, dbus::Error>;
    fn private_devices(&self) -> Result<bool, dbus::Error>;
    fn protect_clock(&self) -> Result<bool, dbus::Error>;
    fn protect_kernel_tunables(&self) -> Result<bool, dbus::Error>;
    fn protect_kernel_modules(&self) -> Result<bool, dbus::Error>;
    fn protect_kernel_logs(&self) -> Result<bool, dbus::Error>;
    fn protect_control_groups(&self) -> Result<bool, dbus::Error>;
    fn protect_control_groups_ex(&self) -> Result<String, dbus::Error>;
    fn private_network(&self) -> Result<bool, dbus::Error>;
    fn private_users(&self) -> Result<bool, dbus::Error>;
    fn private_users_ex(&self) -> Result<String, dbus::Error>;
    fn private_mounts(&self) -> Result<bool, dbus::Error>;
    fn private_ipc(&self) -> Result<bool, dbus::Error>;
    fn private_pids(&self) -> Result<String, dbus::Error>;
    fn protect_home(&self) -> Result<String, dbus::Error>;
    fn protect_system(&self) -> Result<String, dbus::Error>;
    fn same_process_group(&self) -> Result<bool, dbus::Error>;
    fn utmp_identifier(&self) -> Result<String, dbus::Error>;
    fn utmp_mode(&self) -> Result<String, dbus::Error>;
    fn selinux_context(&self) -> Result<(bool, String,), dbus::Error>;
    fn app_armor_profile(&self) -> Result<(bool, String,), dbus::Error>;
    fn smack_process_label(&self) -> Result<(bool, String,), dbus::Error>;
    fn ignore_sigpipe(&self) -> Result<bool, dbus::Error>;
    fn no_new_privileges(&self) -> Result<bool, dbus::Error>;
    fn system_call_filter(&self) -> Result<(bool, Vec<String>,), dbus::Error>;
    fn system_call_architectures(&self) -> Result<Vec<String>, dbus::Error>;
    fn system_call_error_number(&self) -> Result<i32, dbus::Error>;
    fn system_call_log(&self) -> Result<(bool, Vec<String>,), dbus::Error>;
    fn personality(&self) -> Result<String, dbus::Error>;
    fn lock_personality(&self) -> Result<bool, dbus::Error>;
    fn restrict_address_families(&self) -> Result<(bool, Vec<String>,), dbus::Error>;
    fn runtime_directory_symlink(&self) -> Result<Vec<(String, String, u64,)>, dbus::Error>;
    fn runtime_directory_preserve(&self) -> Result<String, dbus::Error>;
    fn runtime_directory_mode(&self) -> Result<u32, dbus::Error>;
    fn runtime_directory(&self) -> Result<Vec<String>, dbus::Error>;
    fn state_directory_symlink(&self) -> Result<Vec<(String, String, u64,)>, dbus::Error>;
    fn state_directory_mode(&self) -> Result<u32, dbus::Error>;
    fn state_directory(&self) -> Result<Vec<String>, dbus::Error>;
    fn cache_directory_symlink(&self) -> Result<Vec<(String, String, u64,)>, dbus::Error>;
    fn cache_directory_mode(&self) -> Result<u32, dbus::Error>;
    fn cache_directory(&self) -> Result<Vec<String>, dbus::Error>;
    fn logs_directory_symlink(&self) -> Result<Vec<(String, String, u64,)>, dbus::Error>;
    fn logs_directory_mode(&self) -> Result<u32, dbus::Error>;
    fn logs_directory(&self) -> Result<Vec<String>, dbus::Error>;
    fn configuration_directory_mode(&self) -> Result<u32, dbus::Error>;
    fn configuration_directory(&self) -> Result<Vec<String>, dbus::Error>;
    fn timeout_clean_usec(&self) -> Result<u64, dbus::Error>;
    fn memory_deny_write_execute(&self) -> Result<bool, dbus::Error>;
    fn restrict_realtime(&self) -> Result<bool, dbus::Error>;
    fn restrict_suidsgid(&self) -> Result<bool, dbus::Error>;
    fn restrict_namespaces(&self) -> Result<u64, dbus::Error>;
    fn restrict_file_systems(&self) -> Result<(bool, Vec<String>,), dbus::Error>;
    fn bind_paths(&self) -> Result<Vec<(String, String, bool, u64,)>, dbus::Error>;
    fn bind_read_only_paths(&self) -> Result<Vec<(String, String, bool, u64,)>, dbus::Error>;
    fn temporary_file_system(&self) -> Result<Vec<(String, String,)>, dbus::Error>;
    fn mount_apivfs(&self) -> Result<bool, dbus::Error>;
    fn bind_log_sockets(&self) -> Result<bool, dbus::Error>;
    fn keyring_mode(&self) -> Result<String, dbus::Error>;
    fn protect_proc(&self) -> Result<String, dbus::Error>;
    fn proc_subset(&self) -> Result<String, dbus::Error>;
    fn protect_hostname(&self) -> Result<bool, dbus::Error>;
    fn memory_ksm(&self) -> Result<bool, dbus::Error>;
    fn network_namespace_path(&self) -> Result<String, dbus::Error>;
    fn ipcnamespace_path(&self) -> Result<String, dbus::Error>;
    fn root_image_policy(&self) -> Result<String, dbus::Error>;
    fn mount_image_policy(&self) -> Result<String, dbus::Error>;
    fn extension_image_policy(&self) -> Result<String, dbus::Error>;
    fn kill_mode(&self) -> Result<String, dbus::Error>;
    fn kill_signal(&self) -> Result<i32, dbus::Error>;
    fn restart_kill_signal(&self) -> Result<i32, dbus::Error>;
    fn final_kill_signal(&self) -> Result<i32, dbus::Error>;
    fn send_sigkill(&self) -> Result<bool, dbus::Error>;
    fn send_sighup(&self) -> Result<bool, dbus::Error>;
    fn watchdog_signal(&self) -> Result<i32, dbus::Error>;
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target=T>> OrgFreedesktopSystemd1Service for blocking::Proxy<'a, C> {

    fn bind_mount(&self, source: &str, destination: &str, read_only: bool, mkdir: bool) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.systemd1.Service", "BindMount", (source, destination, read_only, mkdir, ))
    }

    fn mount_image(&self, source: &str, destination: &str, read_only: bool, mkdir: bool, options: Vec<(&str, &str,)>) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.systemd1.Service", "MountImage", (source, destination, read_only, mkdir, options, ))
    }

    fn dump_file_descriptor_store(&self) -> Result<Vec<(String, u32, u32, u32, u64, u32, u32, String, u32,)>, dbus::Error> {
        self.method_call("org.freedesktop.systemd1.Service", "DumpFileDescriptorStore", ())
            .and_then(|r: (Vec<(String, u32, u32, u32, u64, u32, u32, String, u32,)>, )| Ok(r.0, ))
    }

    fn get_processes(&self) -> Result<Vec<(String, u32, String,)>, dbus::Error> {
        self.method_call("org.freedesktop.systemd1.Service", "GetProcesses", ())
            .and_then(|r: (Vec<(String, u32, String,)>, )| Ok(r.0, ))
    }

    fn attach_processes(&self, subcgroup: &str, pids: Vec<u32>) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.systemd1.Service", "AttachProcesses", (subcgroup, pids, ))
    }

    fn type_(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "Type")
    }

    fn exit_type(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "ExitType")
    }

    fn restart(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "Restart")
    }

    fn restart_mode(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "RestartMode")
    }

    fn pidfile(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "PIDFile")
    }

    fn notify_access(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "NotifyAccess")
    }

    fn restart_usec(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "RestartUSec")
    }

    fn restart_steps(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "RestartSteps")
    }

    fn restart_max_delay_usec(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "RestartMaxDelayUSec")
    }

    fn restart_usec_next(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "RestartUSecNext")
    }

    fn timeout_start_usec(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "TimeoutStartUSec")
    }

    fn timeout_stop_usec(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "TimeoutStopUSec")
    }

    fn timeout_abort_usec(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "TimeoutAbortUSec")
    }

    fn timeout_start_failure_mode(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "TimeoutStartFailureMode")
    }

    fn timeout_stop_failure_mode(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "TimeoutStopFailureMode")
    }

    fn runtime_max_usec(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "RuntimeMaxUSec")
    }

    fn runtime_randomized_extra_usec(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "RuntimeRandomizedExtraUSec")
    }

    fn watchdog_usec(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "WatchdogUSec")
    }

    fn watchdog_timestamp(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "WatchdogTimestamp")
    }

    fn watchdog_timestamp_monotonic(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "WatchdogTimestampMonotonic")
    }

    fn root_directory_start_only(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "RootDirectoryStartOnly")
    }

    fn remain_after_exit(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "RemainAfterExit")
    }

    fn guess_main_pid(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "GuessMainPID")
    }

    fn restart_prevent_exit_status(&self) -> Result<(Vec<i32>, Vec<i32>,), dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "RestartPreventExitStatus")
    }

    fn restart_force_exit_status(&self) -> Result<(Vec<i32>, Vec<i32>,), dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "RestartForceExitStatus")
    }

    fn success_exit_status(&self) -> Result<(Vec<i32>, Vec<i32>,), dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "SuccessExitStatus")
    }

    fn main_pid(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "MainPID")
    }

    fn control_pid(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "ControlPID")
    }

    fn bus_name(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "BusName")
    }

    fn file_descriptor_store_max(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "FileDescriptorStoreMax")
    }

    fn nfile_descriptor_store(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "NFileDescriptorStore")
    }

    fn file_descriptor_store_preserve(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "FileDescriptorStorePreserve")
    }

    fn status_text(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "StatusText")
    }

    fn status_errno(&self) -> Result<i32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "StatusErrno")
    }

    fn status_bus_error(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "StatusBusError")
    }

    fn status_varlink_error(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "StatusVarlinkError")
    }

    fn result(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "Result")
    }

    fn reload_result(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "ReloadResult")
    }

    fn clean_result(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "CleanResult")
    }

    fn live_mount_result(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "LiveMountResult")
    }

    fn usbfunction_descriptors(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "USBFunctionDescriptors")
    }

    fn usbfunction_strings(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "USBFunctionStrings")
    }

    fn uid(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "UID")
    }

    fn gid(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "GID")
    }

    fn nrestarts(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "NRestarts")
    }

    fn oompolicy(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "OOMPolicy")
    }

    fn open_file(&self) -> Result<Vec<(String, String, u64,)>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "OpenFile")
    }

    fn extra_file_descriptor_names(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "ExtraFileDescriptorNames")
    }

    fn reload_signal(&self) -> Result<i32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "ReloadSignal")
    }

    fn exec_main_start_timestamp(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "ExecMainStartTimestamp")
    }

    fn exec_main_start_timestamp_monotonic(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "ExecMainStartTimestampMonotonic")
    }

    fn exec_main_exit_timestamp(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "ExecMainExitTimestamp")
    }

    fn exec_main_exit_timestamp_monotonic(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "ExecMainExitTimestampMonotonic")
    }

    fn exec_main_handoff_timestamp(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "ExecMainHandoffTimestamp")
    }

    fn exec_main_handoff_timestamp_monotonic(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "ExecMainHandoffTimestampMonotonic")
    }

    fn exec_main_pid(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "ExecMainPID")
    }

    fn exec_main_code(&self) -> Result<i32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "ExecMainCode")
    }

    fn exec_main_status(&self) -> Result<i32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "ExecMainStatus")
    }

    fn exec_condition(&self) -> Result<Vec<(String, Vec<String>, bool, u64, u64, u64, u64, u32, i32, i32,)>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "ExecCondition")
    }

    fn exec_condition_ex(&self) -> Result<Vec<(String, Vec<String>, Vec<String>, u64, u64, u64, u64, u32, i32, i32,)>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "ExecConditionEx")
    }

    fn exec_start_pre(&self) -> Result<Vec<(String, Vec<String>, bool, u64, u64, u64, u64, u32, i32, i32,)>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "ExecStartPre")
    }

    fn exec_start_pre_ex(&self) -> Result<Vec<(String, Vec<String>, Vec<String>, u64, u64, u64, u64, u32, i32, i32,)>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "ExecStartPreEx")
    }

    fn exec_start(&self) -> Result<Vec<(String, Vec<String>, bool, u64, u64, u64, u64, u32, i32, i32,)>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "ExecStart")
    }

    fn exec_start_ex(&self) -> Result<Vec<(String, Vec<String>, Vec<String>, u64, u64, u64, u64, u32, i32, i32,)>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "ExecStartEx")
    }

    fn exec_start_post(&self) -> Result<Vec<(String, Vec<String>, bool, u64, u64, u64, u64, u32, i32, i32,)>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "ExecStartPost")
    }

    fn exec_start_post_ex(&self) -> Result<Vec<(String, Vec<String>, Vec<String>, u64, u64, u64, u64, u32, i32, i32,)>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "ExecStartPostEx")
    }

    fn exec_reload(&self) -> Result<Vec<(String, Vec<String>, bool, u64, u64, u64, u64, u32, i32, i32,)>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "ExecReload")
    }

    fn exec_reload_ex(&self) -> Result<Vec<(String, Vec<String>, Vec<String>, u64, u64, u64, u64, u32, i32, i32,)>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "ExecReloadEx")
    }

    fn exec_stop(&self) -> Result<Vec<(String, Vec<String>, bool, u64, u64, u64, u64, u32, i32, i32,)>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "ExecStop")
    }

    fn exec_stop_ex(&self) -> Result<Vec<(String, Vec<String>, Vec<String>, u64, u64, u64, u64, u32, i32, i32,)>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "ExecStopEx")
    }

    fn exec_stop_post(&self) -> Result<Vec<(String, Vec<String>, bool, u64, u64, u64, u64, u32, i32, i32,)>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "ExecStopPost")
    }

    fn exec_stop_post_ex(&self) -> Result<Vec<(String, Vec<String>, Vec<String>, u64, u64, u64, u64, u32, i32, i32,)>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "ExecStopPostEx")
    }

    fn slice(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "Slice")
    }

    fn control_group(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "ControlGroup")
    }

    fn control_group_id(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "ControlGroupId")
    }

    fn memory_current(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "MemoryCurrent")
    }

    fn memory_peak(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "MemoryPeak")
    }

    fn memory_swap_current(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "MemorySwapCurrent")
    }

    fn memory_swap_peak(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "MemorySwapPeak")
    }

    fn memory_zswap_current(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "MemoryZSwapCurrent")
    }

    fn memory_available(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "MemoryAvailable")
    }

    fn effective_memory_max(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "EffectiveMemoryMax")
    }

    fn effective_memory_high(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "EffectiveMemoryHigh")
    }

    fn cpuusage_nsec(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "CPUUsageNSec")
    }

    fn effective_cpus(&self) -> Result<Vec<u8>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "EffectiveCPUs")
    }

    fn effective_memory_nodes(&self) -> Result<Vec<u8>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "EffectiveMemoryNodes")
    }

    fn tasks_current(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "TasksCurrent")
    }

    fn effective_tasks_max(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "EffectiveTasksMax")
    }

    fn ipingress_bytes(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "IPIngressBytes")
    }

    fn ipingress_packets(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "IPIngressPackets")
    }

    fn ipegress_bytes(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "IPEgressBytes")
    }

    fn ipegress_packets(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "IPEgressPackets")
    }

    fn ioread_bytes(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "IOReadBytes")
    }

    fn ioread_operations(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "IOReadOperations")
    }

    fn iowrite_bytes(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "IOWriteBytes")
    }

    fn iowrite_operations(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "IOWriteOperations")
    }

    fn delegate(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "Delegate")
    }

    fn delegate_controllers(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "DelegateControllers")
    }

    fn delegate_subgroup(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "DelegateSubgroup")
    }

    fn cpuaccounting(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "CPUAccounting")
    }

    fn cpuweight(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "CPUWeight")
    }

    fn startup_cpuweight(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "StartupCPUWeight")
    }

    fn cpushares(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "CPUShares")
    }

    fn startup_cpushares(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "StartupCPUShares")
    }

    fn cpuquota_per_sec_usec(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "CPUQuotaPerSecUSec")
    }

    fn cpuquota_period_usec(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "CPUQuotaPeriodUSec")
    }

    fn allowed_cpus(&self) -> Result<Vec<u8>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "AllowedCPUs")
    }

    fn startup_allowed_cpus(&self) -> Result<Vec<u8>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "StartupAllowedCPUs")
    }

    fn allowed_memory_nodes(&self) -> Result<Vec<u8>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "AllowedMemoryNodes")
    }

    fn startup_allowed_memory_nodes(&self) -> Result<Vec<u8>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "StartupAllowedMemoryNodes")
    }

    fn ioaccounting(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "IOAccounting")
    }

    fn ioweight(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "IOWeight")
    }

    fn startup_ioweight(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "StartupIOWeight")
    }

    fn iodevice_weight(&self) -> Result<Vec<(String, u64,)>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "IODeviceWeight")
    }

    fn ioread_bandwidth_max(&self) -> Result<Vec<(String, u64,)>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "IOReadBandwidthMax")
    }

    fn iowrite_bandwidth_max(&self) -> Result<Vec<(String, u64,)>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "IOWriteBandwidthMax")
    }

    fn ioread_iopsmax(&self) -> Result<Vec<(String, u64,)>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "IOReadIOPSMax")
    }

    fn iowrite_iopsmax(&self) -> Result<Vec<(String, u64,)>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "IOWriteIOPSMax")
    }

    fn iodevice_latency_target_usec(&self) -> Result<Vec<(String, u64,)>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "IODeviceLatencyTargetUSec")
    }

    fn block_ioaccounting(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "BlockIOAccounting")
    }

    fn block_ioweight(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "BlockIOWeight")
    }

    fn startup_block_ioweight(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "StartupBlockIOWeight")
    }

    fn block_iodevice_weight(&self) -> Result<Vec<(String, u64,)>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "BlockIODeviceWeight")
    }

    fn block_ioread_bandwidth(&self) -> Result<Vec<(String, u64,)>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "BlockIOReadBandwidth")
    }

    fn block_iowrite_bandwidth(&self) -> Result<Vec<(String, u64,)>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "BlockIOWriteBandwidth")
    }

    fn memory_accounting(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "MemoryAccounting")
    }

    fn default_memory_low(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "DefaultMemoryLow")
    }

    fn default_startup_memory_low(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "DefaultStartupMemoryLow")
    }

    fn default_memory_min(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "DefaultMemoryMin")
    }

    fn memory_min(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "MemoryMin")
    }

    fn memory_low(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "MemoryLow")
    }

    fn startup_memory_low(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "StartupMemoryLow")
    }

    fn memory_high(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "MemoryHigh")
    }

    fn startup_memory_high(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "StartupMemoryHigh")
    }

    fn memory_max(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "MemoryMax")
    }

    fn startup_memory_max(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "StartupMemoryMax")
    }

    fn memory_swap_max(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "MemorySwapMax")
    }

    fn startup_memory_swap_max(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "StartupMemorySwapMax")
    }

    fn memory_zswap_max(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "MemoryZSwapMax")
    }

    fn startup_memory_zswap_max(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "StartupMemoryZSwapMax")
    }

    fn memory_zswap_writeback(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "MemoryZSwapWriteback")
    }

    fn memory_limit(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "MemoryLimit")
    }

    fn device_policy(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "DevicePolicy")
    }

    fn device_allow(&self) -> Result<Vec<(String, String,)>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "DeviceAllow")
    }

    fn tasks_accounting(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "TasksAccounting")
    }

    fn tasks_max(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "TasksMax")
    }

    fn ipaccounting(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "IPAccounting")
    }

    fn ipaddress_allow(&self) -> Result<Vec<(i32, Vec<u8>, u32,)>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "IPAddressAllow")
    }

    fn ipaddress_deny(&self) -> Result<Vec<(i32, Vec<u8>, u32,)>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "IPAddressDeny")
    }

    fn ipingress_filter_path(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "IPIngressFilterPath")
    }

    fn ipegress_filter_path(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "IPEgressFilterPath")
    }

    fn disable_controllers(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "DisableControllers")
    }

    fn managed_oomswap(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "ManagedOOMSwap")
    }

    fn managed_oommemory_pressure(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "ManagedOOMMemoryPressure")
    }

    fn managed_oommemory_pressure_limit(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "ManagedOOMMemoryPressureLimit")
    }

    fn managed_oommemory_pressure_duration_usec(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "ManagedOOMMemoryPressureDurationUSec")
    }

    fn managed_oompreference(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "ManagedOOMPreference")
    }

    fn bpfprogram(&self) -> Result<Vec<(String, String,)>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "BPFProgram")
    }

    fn socket_bind_allow(&self) -> Result<Vec<(i32, i32, u16, u16,)>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "SocketBindAllow")
    }

    fn socket_bind_deny(&self) -> Result<Vec<(i32, i32, u16, u16,)>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "SocketBindDeny")
    }

    fn restrict_network_interfaces(&self) -> Result<(bool, Vec<String>,), dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "RestrictNetworkInterfaces")
    }

    fn memory_pressure_watch(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "MemoryPressureWatch")
    }

    fn memory_pressure_threshold_usec(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "MemoryPressureThresholdUSec")
    }

    fn nftset(&self) -> Result<Vec<(i32, i32, String, String,)>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "NFTSet")
    }

    fn coredump_receive(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "CoredumpReceive")
    }

    fn environment(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "Environment")
    }

    fn environment_files(&self) -> Result<Vec<(String, bool,)>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "EnvironmentFiles")
    }

    fn pass_environment(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "PassEnvironment")
    }

    fn unset_environment(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "UnsetEnvironment")
    }

    fn umask(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "UMask")
    }

    fn limit_cpu(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "LimitCPU")
    }

    fn limit_cpusoft(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "LimitCPUSoft")
    }

    fn limit_fsize(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "LimitFSIZE")
    }

    fn limit_fsizesoft(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "LimitFSIZESoft")
    }

    fn limit_data(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "LimitDATA")
    }

    fn limit_datasoft(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "LimitDATASoft")
    }

    fn limit_stack(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "LimitSTACK")
    }

    fn limit_stacksoft(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "LimitSTACKSoft")
    }

    fn limit_core(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "LimitCORE")
    }

    fn limit_coresoft(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "LimitCORESoft")
    }

    fn limit_rss(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "LimitRSS")
    }

    fn limit_rsssoft(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "LimitRSSSoft")
    }

    fn limit_nofile(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "LimitNOFILE")
    }

    fn limit_nofilesoft(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "LimitNOFILESoft")
    }

    fn limit_as(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "LimitAS")
    }

    fn limit_assoft(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "LimitASSoft")
    }

    fn limit_nproc(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "LimitNPROC")
    }

    fn limit_nprocsoft(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "LimitNPROCSoft")
    }

    fn limit_memlock(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "LimitMEMLOCK")
    }

    fn limit_memlocksoft(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "LimitMEMLOCKSoft")
    }

    fn limit_locks(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "LimitLOCKS")
    }

    fn limit_lockssoft(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "LimitLOCKSSoft")
    }

    fn limit_sigpending(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "LimitSIGPENDING")
    }

    fn limit_sigpendingsoft(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "LimitSIGPENDINGSoft")
    }

    fn limit_msgqueue(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "LimitMSGQUEUE")
    }

    fn limit_msgqueuesoft(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "LimitMSGQUEUESoft")
    }

    fn limit_nice(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "LimitNICE")
    }

    fn limit_nicesoft(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "LimitNICESoft")
    }

    fn limit_rtprio(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "LimitRTPRIO")
    }

    fn limit_rtpriosoft(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "LimitRTPRIOSoft")
    }

    fn limit_rttime(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "LimitRTTIME")
    }

    fn limit_rttimesoft(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "LimitRTTIMESoft")
    }

    fn working_directory(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "WorkingDirectory")
    }

    fn root_directory(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "RootDirectory")
    }

    fn root_image(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "RootImage")
    }

    fn root_image_options(&self) -> Result<Vec<(String, String,)>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "RootImageOptions")
    }

    fn root_hash(&self) -> Result<Vec<u8>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "RootHash")
    }

    fn root_hash_path(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "RootHashPath")
    }

    fn root_hash_signature(&self) -> Result<Vec<u8>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "RootHashSignature")
    }

    fn root_hash_signature_path(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "RootHashSignaturePath")
    }

    fn root_verity(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "RootVerity")
    }

    fn root_ephemeral(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "RootEphemeral")
    }

    fn extension_directories(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "ExtensionDirectories")
    }

    fn extension_images(&self) -> Result<Vec<(String, bool, Vec<(String, String,)>,)>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "ExtensionImages")
    }

    fn mount_images(&self) -> Result<Vec<(String, String, bool, Vec<(String, String,)>,)>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "MountImages")
    }

    fn oomscore_adjust(&self) -> Result<i32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "OOMScoreAdjust")
    }

    fn coredump_filter(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "CoredumpFilter")
    }

    fn nice(&self) -> Result<i32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "Nice")
    }

    fn ioscheduling_class(&self) -> Result<i32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "IOSchedulingClass")
    }

    fn ioscheduling_priority(&self) -> Result<i32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "IOSchedulingPriority")
    }

    fn cpuscheduling_policy(&self) -> Result<i32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "CPUSchedulingPolicy")
    }

    fn cpuscheduling_priority(&self) -> Result<i32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "CPUSchedulingPriority")
    }

    fn cpuaffinity(&self) -> Result<Vec<u8>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "CPUAffinity")
    }

    fn cpuaffinity_from_numa(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "CPUAffinityFromNUMA")
    }

    fn numapolicy(&self) -> Result<i32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "NUMAPolicy")
    }

    fn numamask(&self) -> Result<Vec<u8>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "NUMAMask")
    }

    fn timer_slack_nsec(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "TimerSlackNSec")
    }

    fn cpuscheduling_reset_on_fork(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "CPUSchedulingResetOnFork")
    }

    fn non_blocking(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "NonBlocking")
    }

    fn standard_input(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "StandardInput")
    }

    fn standard_input_file_descriptor_name(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "StandardInputFileDescriptorName")
    }

    fn standard_input_data(&self) -> Result<Vec<u8>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "StandardInputData")
    }

    fn standard_output(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "StandardOutput")
    }

    fn standard_output_file_descriptor_name(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "StandardOutputFileDescriptorName")
    }

    fn standard_error(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "StandardError")
    }

    fn standard_error_file_descriptor_name(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "StandardErrorFileDescriptorName")
    }

    fn ttypath(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "TTYPath")
    }

    fn ttyreset(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "TTYReset")
    }

    fn ttyvhangup(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "TTYVHangup")
    }

    fn ttyvtdisallocate(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "TTYVTDisallocate")
    }

    fn ttyrows(&self) -> Result<u16, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "TTYRows")
    }

    fn ttycolumns(&self) -> Result<u16, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "TTYColumns")
    }

    fn syslog_priority(&self) -> Result<i32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "SyslogPriority")
    }

    fn syslog_identifier(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "SyslogIdentifier")
    }

    fn syslog_level_prefix(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "SyslogLevelPrefix")
    }

    fn syslog_level(&self) -> Result<i32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "SyslogLevel")
    }

    fn syslog_facility(&self) -> Result<i32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "SyslogFacility")
    }

    fn log_level_max(&self) -> Result<i32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "LogLevelMax")
    }

    fn log_rate_limit_interval_usec(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "LogRateLimitIntervalUSec")
    }

    fn log_rate_limit_burst(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "LogRateLimitBurst")
    }

    fn log_extra_fields(&self) -> Result<Vec<Vec<u8>>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "LogExtraFields")
    }

    fn log_filter_patterns(&self) -> Result<Vec<(bool, String,)>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "LogFilterPatterns")
    }

    fn log_namespace(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "LogNamespace")
    }

    fn secure_bits(&self) -> Result<i32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "SecureBits")
    }

    fn capability_bounding_set(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "CapabilityBoundingSet")
    }

    fn ambient_capabilities(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "AmbientCapabilities")
    }

    fn user(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "User")
    }

    fn group(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "Group")
    }

    fn dynamic_user(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "DynamicUser")
    }

    fn set_login_environment(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "SetLoginEnvironment")
    }

    fn remove_ipc(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "RemoveIPC")
    }

    fn set_credential(&self) -> Result<Vec<(String, Vec<u8>,)>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "SetCredential")
    }

    fn set_credential_encrypted(&self) -> Result<Vec<(String, Vec<u8>,)>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "SetCredentialEncrypted")
    }

    fn load_credential(&self) -> Result<Vec<(String, String,)>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "LoadCredential")
    }

    fn load_credential_encrypted(&self) -> Result<Vec<(String, String,)>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "LoadCredentialEncrypted")
    }

    fn import_credential(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "ImportCredential")
    }

    fn import_credential_ex(&self) -> Result<Vec<(String, String,)>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "ImportCredentialEx")
    }

    fn supplementary_groups(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "SupplementaryGroups")
    }

    fn pamname(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "PAMName")
    }

    fn read_write_paths(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "ReadWritePaths")
    }

    fn read_only_paths(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "ReadOnlyPaths")
    }

    fn inaccessible_paths(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "InaccessiblePaths")
    }

    fn exec_paths(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "ExecPaths")
    }

    fn no_exec_paths(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "NoExecPaths")
    }

    fn exec_search_path(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "ExecSearchPath")
    }

    fn mount_flags(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "MountFlags")
    }

    fn private_tmp(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "PrivateTmp")
    }

    fn private_tmp_ex(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "PrivateTmpEx")
    }

    fn private_devices(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "PrivateDevices")
    }

    fn protect_clock(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "ProtectClock")
    }

    fn protect_kernel_tunables(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "ProtectKernelTunables")
    }

    fn protect_kernel_modules(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "ProtectKernelModules")
    }

    fn protect_kernel_logs(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "ProtectKernelLogs")
    }

    fn protect_control_groups(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "ProtectControlGroups")
    }

    fn protect_control_groups_ex(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "ProtectControlGroupsEx")
    }

    fn private_network(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "PrivateNetwork")
    }

    fn private_users(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "PrivateUsers")
    }

    fn private_users_ex(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "PrivateUsersEx")
    }

    fn private_mounts(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "PrivateMounts")
    }

    fn private_ipc(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "PrivateIPC")
    }

    fn private_pids(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "PrivatePIDs")
    }

    fn protect_home(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "ProtectHome")
    }

    fn protect_system(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "ProtectSystem")
    }

    fn same_process_group(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "SameProcessGroup")
    }

    fn utmp_identifier(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "UtmpIdentifier")
    }

    fn utmp_mode(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "UtmpMode")
    }

    fn selinux_context(&self) -> Result<(bool, String,), dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "SELinuxContext")
    }

    fn app_armor_profile(&self) -> Result<(bool, String,), dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "AppArmorProfile")
    }

    fn smack_process_label(&self) -> Result<(bool, String,), dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "SmackProcessLabel")
    }

    fn ignore_sigpipe(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "IgnoreSIGPIPE")
    }

    fn no_new_privileges(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "NoNewPrivileges")
    }

    fn system_call_filter(&self) -> Result<(bool, Vec<String>,), dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "SystemCallFilter")
    }

    fn system_call_architectures(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "SystemCallArchitectures")
    }

    fn system_call_error_number(&self) -> Result<i32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "SystemCallErrorNumber")
    }

    fn system_call_log(&self) -> Result<(bool, Vec<String>,), dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "SystemCallLog")
    }

    fn personality(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "Personality")
    }

    fn lock_personality(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "LockPersonality")
    }

    fn restrict_address_families(&self) -> Result<(bool, Vec<String>,), dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "RestrictAddressFamilies")
    }

    fn runtime_directory_symlink(&self) -> Result<Vec<(String, String, u64,)>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "RuntimeDirectorySymlink")
    }

    fn runtime_directory_preserve(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "RuntimeDirectoryPreserve")
    }

    fn runtime_directory_mode(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "RuntimeDirectoryMode")
    }

    fn runtime_directory(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "RuntimeDirectory")
    }

    fn state_directory_symlink(&self) -> Result<Vec<(String, String, u64,)>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "StateDirectorySymlink")
    }

    fn state_directory_mode(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "StateDirectoryMode")
    }

    fn state_directory(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "StateDirectory")
    }

    fn cache_directory_symlink(&self) -> Result<Vec<(String, String, u64,)>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "CacheDirectorySymlink")
    }

    fn cache_directory_mode(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "CacheDirectoryMode")
    }

    fn cache_directory(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "CacheDirectory")
    }

    fn logs_directory_symlink(&self) -> Result<Vec<(String, String, u64,)>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "LogsDirectorySymlink")
    }

    fn logs_directory_mode(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "LogsDirectoryMode")
    }

    fn logs_directory(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "LogsDirectory")
    }

    fn configuration_directory_mode(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "ConfigurationDirectoryMode")
    }

    fn configuration_directory(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "ConfigurationDirectory")
    }

    fn timeout_clean_usec(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "TimeoutCleanUSec")
    }

    fn memory_deny_write_execute(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "MemoryDenyWriteExecute")
    }

    fn restrict_realtime(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "RestrictRealtime")
    }

    fn restrict_suidsgid(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "RestrictSUIDSGID")
    }

    fn restrict_namespaces(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "RestrictNamespaces")
    }

    fn restrict_file_systems(&self) -> Result<(bool, Vec<String>,), dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "RestrictFileSystems")
    }

    fn bind_paths(&self) -> Result<Vec<(String, String, bool, u64,)>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "BindPaths")
    }

    fn bind_read_only_paths(&self) -> Result<Vec<(String, String, bool, u64,)>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "BindReadOnlyPaths")
    }

    fn temporary_file_system(&self) -> Result<Vec<(String, String,)>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "TemporaryFileSystem")
    }

    fn mount_apivfs(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "MountAPIVFS")
    }

    fn bind_log_sockets(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "BindLogSockets")
    }

    fn keyring_mode(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "KeyringMode")
    }

    fn protect_proc(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "ProtectProc")
    }

    fn proc_subset(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "ProcSubset")
    }

    fn protect_hostname(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "ProtectHostname")
    }

    fn memory_ksm(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "MemoryKSM")
    }

    fn network_namespace_path(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "NetworkNamespacePath")
    }

    fn ipcnamespace_path(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "IPCNamespacePath")
    }

    fn root_image_policy(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "RootImagePolicy")
    }

    fn mount_image_policy(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "MountImagePolicy")
    }

    fn extension_image_policy(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "ExtensionImagePolicy")
    }

    fn kill_mode(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "KillMode")
    }

    fn kill_signal(&self) -> Result<i32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "KillSignal")
    }

    fn restart_kill_signal(&self) -> Result<i32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "RestartKillSignal")
    }

    fn final_kill_signal(&self) -> Result<i32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "FinalKillSignal")
    }

    fn send_sigkill(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "SendSIGKILL")
    }

    fn send_sighup(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "SendSIGHUP")
    }

    fn watchdog_signal(&self) -> Result<i32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Service", "WatchdogSignal")
    }
}

pub trait OrgFreedesktopSystemd1Unit {
    fn start(&self, mode: &str) -> Result<dbus::Path<'static>, dbus::Error>;
    fn stop(&self, mode: &str) -> Result<dbus::Path<'static>, dbus::Error>;
    fn reload(&self, mode: &str) -> Result<dbus::Path<'static>, dbus::Error>;
    fn restart(&self, mode: &str) -> Result<dbus::Path<'static>, dbus::Error>;
    fn try_restart(&self, mode: &str) -> Result<dbus::Path<'static>, dbus::Error>;
    fn reload_or_restart(&self, mode: &str) -> Result<dbus::Path<'static>, dbus::Error>;
    fn reload_or_try_restart(&self, mode: &str) -> Result<dbus::Path<'static>, dbus::Error>;
    fn enqueue_job(&self, job_type: &str, job_mode: &str) -> Result<(u32, dbus::Path<'static>, String, dbus::Path<'static>, String, Vec<(u32, dbus::Path<'static>, String, dbus::Path<'static>, String,)>), dbus::Error>;
    fn kill(&self, whom: &str, signal: i32) -> Result<(), dbus::Error>;
    fn queue_signal(&self, whom: &str, signal: i32, value: i32) -> Result<(), dbus::Error>;
    fn reset_failed(&self) -> Result<(), dbus::Error>;
    fn set_properties(&self, runtime: bool, properties: Vec<(&str, arg::Variant<Box<dyn arg::RefArg>>,)>) -> Result<(), dbus::Error>;
    fn ref_(&self) -> Result<(), dbus::Error>;
    fn unref(&self) -> Result<(), dbus::Error>;
    fn clean(&self, mask: Vec<&str>) -> Result<(), dbus::Error>;
    fn freeze(&self) -> Result<(), dbus::Error>;
    fn thaw(&self) -> Result<(), dbus::Error>;
    fn id(&self) -> Result<String, dbus::Error>;
    fn names(&self) -> Result<Vec<String>, dbus::Error>;
    fn following(&self) -> Result<String, dbus::Error>;
    fn requires(&self) -> Result<Vec<String>, dbus::Error>;
    fn requisite(&self) -> Result<Vec<String>, dbus::Error>;
    fn wants(&self) -> Result<Vec<String>, dbus::Error>;
    fn binds_to(&self) -> Result<Vec<String>, dbus::Error>;
    fn part_of(&self) -> Result<Vec<String>, dbus::Error>;
    fn upholds(&self) -> Result<Vec<String>, dbus::Error>;
    fn required_by(&self) -> Result<Vec<String>, dbus::Error>;
    fn requisite_of(&self) -> Result<Vec<String>, dbus::Error>;
    fn wanted_by(&self) -> Result<Vec<String>, dbus::Error>;
    fn bound_by(&self) -> Result<Vec<String>, dbus::Error>;
    fn upheld_by(&self) -> Result<Vec<String>, dbus::Error>;
    fn consists_of(&self) -> Result<Vec<String>, dbus::Error>;
    fn conflicts(&self) -> Result<Vec<String>, dbus::Error>;
    fn conflicted_by(&self) -> Result<Vec<String>, dbus::Error>;
    fn before(&self) -> Result<Vec<String>, dbus::Error>;
    fn after(&self) -> Result<Vec<String>, dbus::Error>;
    fn on_success(&self) -> Result<Vec<String>, dbus::Error>;
    fn on_success_of(&self) -> Result<Vec<String>, dbus::Error>;
    fn on_failure(&self) -> Result<Vec<String>, dbus::Error>;
    fn on_failure_of(&self) -> Result<Vec<String>, dbus::Error>;
    fn triggers(&self) -> Result<Vec<String>, dbus::Error>;
    fn triggered_by(&self) -> Result<Vec<String>, dbus::Error>;
    fn propagates_reload_to(&self) -> Result<Vec<String>, dbus::Error>;
    fn reload_propagated_from(&self) -> Result<Vec<String>, dbus::Error>;
    fn propagates_stop_to(&self) -> Result<Vec<String>, dbus::Error>;
    fn stop_propagated_from(&self) -> Result<Vec<String>, dbus::Error>;
    fn joins_namespace_of(&self) -> Result<Vec<String>, dbus::Error>;
    fn slice_of(&self) -> Result<Vec<String>, dbus::Error>;
    fn requires_mounts_for(&self) -> Result<Vec<String>, dbus::Error>;
    fn wants_mounts_for(&self) -> Result<Vec<String>, dbus::Error>;
    fn documentation(&self) -> Result<Vec<String>, dbus::Error>;
    fn description(&self) -> Result<String, dbus::Error>;
    fn access_selinux_context(&self) -> Result<String, dbus::Error>;
    fn load_state(&self) -> Result<String, dbus::Error>;
    fn active_state(&self) -> Result<String, dbus::Error>;
    fn freezer_state(&self) -> Result<String, dbus::Error>;
    fn sub_state(&self) -> Result<String, dbus::Error>;
    fn fragment_path(&self) -> Result<String, dbus::Error>;
    fn source_path(&self) -> Result<String, dbus::Error>;
    fn drop_in_paths(&self) -> Result<Vec<String>, dbus::Error>;
    fn unit_file_state(&self) -> Result<String, dbus::Error>;
    fn unit_file_preset(&self) -> Result<String, dbus::Error>;
    fn state_change_timestamp(&self) -> Result<u64, dbus::Error>;
    fn state_change_timestamp_monotonic(&self) -> Result<u64, dbus::Error>;
    fn inactive_exit_timestamp(&self) -> Result<u64, dbus::Error>;
    fn inactive_exit_timestamp_monotonic(&self) -> Result<u64, dbus::Error>;
    fn active_enter_timestamp(&self) -> Result<u64, dbus::Error>;
    fn active_enter_timestamp_monotonic(&self) -> Result<u64, dbus::Error>;
    fn active_exit_timestamp(&self) -> Result<u64, dbus::Error>;
    fn active_exit_timestamp_monotonic(&self) -> Result<u64, dbus::Error>;
    fn inactive_enter_timestamp(&self) -> Result<u64, dbus::Error>;
    fn inactive_enter_timestamp_monotonic(&self) -> Result<u64, dbus::Error>;
    fn can_start(&self) -> Result<bool, dbus::Error>;
    fn can_stop(&self) -> Result<bool, dbus::Error>;
    fn can_reload(&self) -> Result<bool, dbus::Error>;
    fn can_isolate(&self) -> Result<bool, dbus::Error>;
    fn can_clean(&self) -> Result<Vec<String>, dbus::Error>;
    fn can_freeze(&self) -> Result<bool, dbus::Error>;
    fn can_live_mount(&self) -> Result<bool, dbus::Error>;
    fn job(&self) -> Result<(u32, dbus::Path<'static>,), dbus::Error>;
    fn stop_when_unneeded(&self) -> Result<bool, dbus::Error>;
    fn refuse_manual_start(&self) -> Result<bool, dbus::Error>;
    fn refuse_manual_stop(&self) -> Result<bool, dbus::Error>;
    fn allow_isolate(&self) -> Result<bool, dbus::Error>;
    fn default_dependencies(&self) -> Result<bool, dbus::Error>;
    fn survive_final_kill_signal(&self) -> Result<bool, dbus::Error>;
    fn on_success_job_mode(&self) -> Result<String, dbus::Error>;
    fn on_failure_job_mode(&self) -> Result<String, dbus::Error>;
    fn ignore_on_isolate(&self) -> Result<bool, dbus::Error>;
    fn need_daemon_reload(&self) -> Result<bool, dbus::Error>;
    fn markers(&self) -> Result<Vec<String>, dbus::Error>;
    fn job_timeout_usec(&self) -> Result<u64, dbus::Error>;
    fn job_running_timeout_usec(&self) -> Result<u64, dbus::Error>;
    fn job_timeout_action(&self) -> Result<String, dbus::Error>;
    fn job_timeout_reboot_argument(&self) -> Result<String, dbus::Error>;
    fn condition_result(&self) -> Result<bool, dbus::Error>;
    fn assert_result(&self) -> Result<bool, dbus::Error>;
    fn condition_timestamp(&self) -> Result<u64, dbus::Error>;
    fn condition_timestamp_monotonic(&self) -> Result<u64, dbus::Error>;
    fn assert_timestamp(&self) -> Result<u64, dbus::Error>;
    fn assert_timestamp_monotonic(&self) -> Result<u64, dbus::Error>;
    fn conditions(&self) -> Result<Vec<(String, bool, bool, String, i32,)>, dbus::Error>;
    fn asserts(&self) -> Result<Vec<(String, bool, bool, String, i32,)>, dbus::Error>;
    fn load_error(&self) -> Result<(String, String,), dbus::Error>;
    fn transient(&self) -> Result<bool, dbus::Error>;
    fn perpetual(&self) -> Result<bool, dbus::Error>;
    fn start_limit_interval_usec(&self) -> Result<u64, dbus::Error>;
    fn start_limit_burst(&self) -> Result<u32, dbus::Error>;
    fn start_limit_action(&self) -> Result<String, dbus::Error>;
    fn failure_action(&self) -> Result<String, dbus::Error>;
    fn failure_action_exit_status(&self) -> Result<i32, dbus::Error>;
    fn success_action(&self) -> Result<String, dbus::Error>;
    fn success_action_exit_status(&self) -> Result<i32, dbus::Error>;
    fn reboot_argument(&self) -> Result<String, dbus::Error>;
    fn invocation_id(&self) -> Result<Vec<u8>, dbus::Error>;
    fn collect_mode(&self) -> Result<String, dbus::Error>;
    fn refs(&self) -> Result<Vec<String>, dbus::Error>;
    fn activation_details(&self) -> Result<Vec<(String, String,)>, dbus::Error>;
    fn debug_invocation(&self) -> Result<bool, dbus::Error>;
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target=T>> OrgFreedesktopSystemd1Unit for blocking::Proxy<'a, C> {

    fn start(&self, mode: &str) -> Result<dbus::Path<'static>, dbus::Error> {
        self.method_call("org.freedesktop.systemd1.Unit", "Start", (mode, ))
            .and_then(|r: (dbus::Path<'static>, )| Ok(r.0, ))
    }

    fn stop(&self, mode: &str) -> Result<dbus::Path<'static>, dbus::Error> {
        self.method_call("org.freedesktop.systemd1.Unit", "Stop", (mode, ))
            .and_then(|r: (dbus::Path<'static>, )| Ok(r.0, ))
    }

    fn reload(&self, mode: &str) -> Result<dbus::Path<'static>, dbus::Error> {
        self.method_call("org.freedesktop.systemd1.Unit", "Reload", (mode, ))
            .and_then(|r: (dbus::Path<'static>, )| Ok(r.0, ))
    }

    fn restart(&self, mode: &str) -> Result<dbus::Path<'static>, dbus::Error> {
        self.method_call("org.freedesktop.systemd1.Unit", "Restart", (mode, ))
            .and_then(|r: (dbus::Path<'static>, )| Ok(r.0, ))
    }

    fn try_restart(&self, mode: &str) -> Result<dbus::Path<'static>, dbus::Error> {
        self.method_call("org.freedesktop.systemd1.Unit", "TryRestart", (mode, ))
            .and_then(|r: (dbus::Path<'static>, )| Ok(r.0, ))
    }

    fn reload_or_restart(&self, mode: &str) -> Result<dbus::Path<'static>, dbus::Error> {
        self.method_call("org.freedesktop.systemd1.Unit", "ReloadOrRestart", (mode, ))
            .and_then(|r: (dbus::Path<'static>, )| Ok(r.0, ))
    }

    fn reload_or_try_restart(&self, mode: &str) -> Result<dbus::Path<'static>, dbus::Error> {
        self.method_call("org.freedesktop.systemd1.Unit", "ReloadOrTryRestart", (mode, ))
            .and_then(|r: (dbus::Path<'static>, )| Ok(r.0, ))
    }

    fn enqueue_job(&self, job_type: &str, job_mode: &str) -> Result<(u32, dbus::Path<'static>, String, dbus::Path<'static>, String, Vec<(u32, dbus::Path<'static>, String, dbus::Path<'static>, String,)>), dbus::Error> {
        self.method_call("org.freedesktop.systemd1.Unit", "EnqueueJob", (job_type, job_mode, ))
    }

    fn kill(&self, whom: &str, signal: i32) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.systemd1.Unit", "Kill", (whom, signal, ))
    }

    fn queue_signal(&self, whom: &str, signal: i32, value: i32) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.systemd1.Unit", "QueueSignal", (whom, signal, value, ))
    }

    fn reset_failed(&self) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.systemd1.Unit", "ResetFailed", ())
    }

    fn set_properties(&self, runtime: bool, properties: Vec<(&str, arg::Variant<Box<dyn arg::RefArg>>,)>) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.systemd1.Unit", "SetProperties", (runtime, properties, ))
    }

    fn ref_(&self) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.systemd1.Unit", "Ref", ())
    }

    fn unref(&self) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.systemd1.Unit", "Unref", ())
    }

    fn clean(&self, mask: Vec<&str>) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.systemd1.Unit", "Clean", (mask, ))
    }

    fn freeze(&self) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.systemd1.Unit", "Freeze", ())
    }

    fn thaw(&self) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.systemd1.Unit", "Thaw", ())
    }

    fn id(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "Id")
    }

    fn names(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "Names")
    }

    fn following(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "Following")
    }

    fn requires(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "Requires")
    }

    fn requisite(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "Requisite")
    }

    fn wants(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "Wants")
    }

    fn binds_to(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "BindsTo")
    }

    fn part_of(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "PartOf")
    }

    fn upholds(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "Upholds")
    }

    fn required_by(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "RequiredBy")
    }

    fn requisite_of(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "RequisiteOf")
    }

    fn wanted_by(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "WantedBy")
    }

    fn bound_by(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "BoundBy")
    }

    fn upheld_by(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "UpheldBy")
    }

    fn consists_of(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "ConsistsOf")
    }

    fn conflicts(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "Conflicts")
    }

    fn conflicted_by(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "ConflictedBy")
    }

    fn before(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "Before")
    }

    fn after(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "After")
    }

    fn on_success(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "OnSuccess")
    }

    fn on_success_of(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "OnSuccessOf")
    }

    fn on_failure(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "OnFailure")
    }

    fn on_failure_of(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "OnFailureOf")
    }

    fn triggers(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "Triggers")
    }

    fn triggered_by(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "TriggeredBy")
    }

    fn propagates_reload_to(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "PropagatesReloadTo")
    }

    fn reload_propagated_from(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "ReloadPropagatedFrom")
    }

    fn propagates_stop_to(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "PropagatesStopTo")
    }

    fn stop_propagated_from(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "StopPropagatedFrom")
    }

    fn joins_namespace_of(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "JoinsNamespaceOf")
    }

    fn slice_of(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "SliceOf")
    }

    fn requires_mounts_for(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "RequiresMountsFor")
    }

    fn wants_mounts_for(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "WantsMountsFor")
    }

    fn documentation(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "Documentation")
    }

    fn description(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "Description")
    }

    fn access_selinux_context(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "AccessSELinuxContext")
    }

    fn load_state(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "LoadState")
    }

    fn active_state(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "ActiveState")
    }

    fn freezer_state(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "FreezerState")
    }

    fn sub_state(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "SubState")
    }

    fn fragment_path(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "FragmentPath")
    }

    fn source_path(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "SourcePath")
    }

    fn drop_in_paths(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "DropInPaths")
    }

    fn unit_file_state(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "UnitFileState")
    }

    fn unit_file_preset(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "UnitFilePreset")
    }

    fn state_change_timestamp(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "StateChangeTimestamp")
    }

    fn state_change_timestamp_monotonic(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "StateChangeTimestampMonotonic")
    }

    fn inactive_exit_timestamp(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "InactiveExitTimestamp")
    }

    fn inactive_exit_timestamp_monotonic(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "InactiveExitTimestampMonotonic")
    }

    fn active_enter_timestamp(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "ActiveEnterTimestamp")
    }

    fn active_enter_timestamp_monotonic(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "ActiveEnterTimestampMonotonic")
    }

    fn active_exit_timestamp(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "ActiveExitTimestamp")
    }

    fn active_exit_timestamp_monotonic(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "ActiveExitTimestampMonotonic")
    }

    fn inactive_enter_timestamp(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "InactiveEnterTimestamp")
    }

    fn inactive_enter_timestamp_monotonic(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "InactiveEnterTimestampMonotonic")
    }

    fn can_start(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "CanStart")
    }

    fn can_stop(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "CanStop")
    }

    fn can_reload(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "CanReload")
    }

    fn can_isolate(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "CanIsolate")
    }

    fn can_clean(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "CanClean")
    }

    fn can_freeze(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "CanFreeze")
    }

    fn can_live_mount(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "CanLiveMount")
    }

    fn job(&self) -> Result<(u32, dbus::Path<'static>,), dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "Job")
    }

    fn stop_when_unneeded(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "StopWhenUnneeded")
    }

    fn refuse_manual_start(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "RefuseManualStart")
    }

    fn refuse_manual_stop(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "RefuseManualStop")
    }

    fn allow_isolate(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "AllowIsolate")
    }

    fn default_dependencies(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "DefaultDependencies")
    }

    fn survive_final_kill_signal(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "SurviveFinalKillSignal")
    }

    fn on_success_job_mode(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "OnSuccessJobMode")
    }

    fn on_failure_job_mode(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "OnFailureJobMode")
    }

    fn ignore_on_isolate(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "IgnoreOnIsolate")
    }

    fn need_daemon_reload(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "NeedDaemonReload")
    }

    fn markers(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "Markers")
    }

    fn job_timeout_usec(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "JobTimeoutUSec")
    }

    fn job_running_timeout_usec(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "JobRunningTimeoutUSec")
    }

    fn job_timeout_action(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "JobTimeoutAction")
    }

    fn job_timeout_reboot_argument(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "JobTimeoutRebootArgument")
    }

    fn condition_result(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "ConditionResult")
    }

    fn assert_result(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "AssertResult")
    }

    fn condition_timestamp(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "ConditionTimestamp")
    }

    fn condition_timestamp_monotonic(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "ConditionTimestampMonotonic")
    }

    fn assert_timestamp(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "AssertTimestamp")
    }

    fn assert_timestamp_monotonic(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "AssertTimestampMonotonic")
    }

    fn conditions(&self) -> Result<Vec<(String, bool, bool, String, i32,)>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "Conditions")
    }

    fn asserts(&self) -> Result<Vec<(String, bool, bool, String, i32,)>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "Asserts")
    }

    fn load_error(&self) -> Result<(String, String,), dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "LoadError")
    }

    fn transient(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "Transient")
    }

    fn perpetual(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "Perpetual")
    }

    fn start_limit_interval_usec(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "StartLimitIntervalUSec")
    }

    fn start_limit_burst(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "StartLimitBurst")
    }

    fn start_limit_action(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "StartLimitAction")
    }

    fn failure_action(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "FailureAction")
    }

    fn failure_action_exit_status(&self) -> Result<i32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "FailureActionExitStatus")
    }

    fn success_action(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "SuccessAction")
    }

    fn success_action_exit_status(&self) -> Result<i32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "SuccessActionExitStatus")
    }

    fn reboot_argument(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "RebootArgument")
    }

    fn invocation_id(&self) -> Result<Vec<u8>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "InvocationID")
    }

    fn collect_mode(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "CollectMode")
    }

    fn refs(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "Refs")
    }

    fn activation_details(&self) -> Result<Vec<(String, String,)>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "ActivationDetails")
    }

    fn debug_invocation(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(self, "org.freedesktop.systemd1.Unit", "DebugInvocation")
    }
}
