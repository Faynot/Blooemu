![](./blooemux.png)
# Wellcome to Blooemu!
Blooemu - is Rust library, for OS API's manipulation. 

You can use 1 and the same code that uses the OS API and it will work on different OS, you don't need to download a bunch of different libraries for each API to make the program work as it should on any OS

Todo:

| Windows      | Linux        | MacOS     |
| -------------| -------------|-----------|
| Supported    | Supported    | Supported |
| **Process Management:** |
| open✅| open✅| open✅     |
| close✅| close✅| close✅     |
| get_pid✅| get_pid✅|get_pid✅|
| get_process_name✅| get_process_name✅|get_process_name✅|
| get_process_memory_usage✅| get_process_memory_usage✅| get_process_memory_usage✅|
| get_process_cpu_usage✅| get_process_cpu_usage✅| get_process_cpu_usage✅|
| get_all_processes✅| get_all_processes✅| get_all_processes✅|
| elevate_privileges🕘| elevate_privileges🕘| elevate_privileges🕘|
| **Network Interaction:** |
| create_socket🕘| create_socket🕘| create_socket🕘|
| bind_socket🕘| bind_socket🕘| bind_socket🕘|
| listen_socket🕘| listen_socket🕘| listen_socket🕘|
| connect_socket🕘| connect_socket🕘| connect_socket🕘|
| send_data🕘| send_data🕘| send_data🕘|
| receive_data🕘| receive_data🕘| receive_data🕘|
| close_socket🕘| close_socket🕘| close_socket🕘|
| is_network_available🕘| is_network_available🕘| is_network_available🕘|
| get_local_ip🕘| get_local_ip🕘| get_local_ip🕘|
| get_external_ip🕘| get_external_ip🕘| get_external_ip🕘|
| resolve_hostname🕘| resolve_hostname🕘| resolve_hostname🕘|
| get_hostname🕘| get_hostname🕘| get_hostname🕘|
| get_network_interfaces🕘| get_network_interfaces🕘| get_network_interfaces🕘|
| get_interface_name🕘| get_interface_name🕘| get_interface_name🕘|
| get_interface_mac_address🕘| get_interface_mac_address🕘| get_interface_mac_address🕘|
| set_interface_ip🕘| set_interface_ip🕘| set_interface_ip🕘|
| set_interface_mtu🕘| set_interface_mtu🕘| set_interface_mtu🕘|
| **File System:** |
| create_file🕘| create_file🕘| create_file🕘|
| open_file🕘| open_file🕘| open_file🕘|
| read_file🕘| read_file🕘| read_file🕘|
| write_file🕘| write_file🕘| write_file🕘|
| close_file🕘| close_file🕘| close_file🕘
| create_directory🕘| create_directory🕘| create_directory🕘|
| delete_directory🕘| delete_directory🕘|           |
| move_directory🕘| move_directory🕘| move_directory🕘|
| get_directory_contents🕘| get_directory_contents🕘| get_directory_contents🕘|
| has_file_access🕘| has_file_access🕘| has_file_access🕘|
| has_directory_access🕘| has_directory_access🕘| has_directory_access🕘|
| get_file_size🕘| get_file_size🕘| get_file_size🕘|
| get_file_creation_date🕘| get_file_creation_date🕘| get_file_creation_date🕘|
| get_file_modification_date🕘| get_file_modification_date🕘| get_file_modification_date🕘|
| get_file_owner🕘| get_file_owner🕘| get_file_owner🕘|
| create_symlink🕘| create_symlink🕘| create_symlink🕘|
| get_symlink_target🕘| get_symlink_target🕘| get_symlink_target🕘|
| **Graphical Interface (GUI):** |
| create_window🕘| create_window🕘| create_window🕘|
| set_window_title🕘| set_window_title🕘| set_window_title🕘|
| set_window_position🕘| set_window_position🕘|| set_window_position🕘|
| set_window_size🕘| set_window_size🕘| set_window_size🕘
| show_window🕘| show_window🕘| show_window🕘|
| hide_window🕘| hide_window🕘| hide_window🕘
| close_window🕘| close_window🕘| close_window🕘|
| create_button🕘| create_button🕘| create_button🕘|
| create_label🕘| create_label🕘| create_label🕘|
| create_text_box🕘| create_text_box🕘| create_text_box🕘|
| create_list_box🕘| create_list_box🕘| create_list_box🕘|
| create_menu🕘| create_menu🕘| create_menu🕘
| register_event_handler🕘| register_event_handler🕘| register_event_handler🕘|
| handle_event🕘| handle_event🕘|           |
| get_event_type🕘| get_event_type🕘| get_event_type🕘|
| get_event_data🕘| get_event_data🕘| get_event_data🕘|
| draw_line🕘| draw_line🕘| draw_line🕘|
| draw_rectangle🕘| draw_rectangle🕘| draw_rectangle🕘|
| draw_circle🕘| draw_circle🕘| draw_circle🕘|
| draw_text🕘| draw_text🕘| draw_text🕘|
| draw_image🕘| draw_image🕘| draw_image🕘|
| get_screen_resolution🕘| get_screen_resolution🕘| get_screen_resolution🕘|
| **Other:** |
| get_operating_system🕘| get_operating_system🕘| get_operating_system🕘|
| get_architecture🕘| get_architecture🕘| get_architecture🕘|
| get_cpu_count🕘| get_cpu_count🕘| get_cpu_count🕘
| get_memory_size🕘| get_memory_size🕘| get_memory_size🕘|
| get_system_time🕘| get_system_time🕘| get_system_time🕘|
| show_notification🕘| show_notification🕘| show_notification🕘|
| play_sound🕘| play_sound🕘| play_sound🕘|
| get_key_state🕘| get_key_state🕘| get_key_state🕘
| get_mouse_position🕘| get_mouse_position🕘| get_mouse_position🕘|
| set_timer🕘| set_timer🕘| set_timer🕘|
| cancel_timer🕘| cancel_timer🕘| cancel_timer🕘|
|alert!✅|alert!✅|alert!🕘|
|error!✅|error!✅|error!🕘|