![](./blooemux.png)
# Welcome to Blooemu!
Blooemu - is Rust library, for OS API's manipulation. 

You can use 1 and the same code that uses the OS API and it will work on different OS, you don't need to download a bunch of different libraries for each API to make the program work as it should on any OS

Todo:

| Windows                        | Linux                       | MacOS                       |
|--------------------------------|-----------------------------|-----------------------------|
| Supported                      | Supported                   | Supported                   |
| **Process Management:**        |
| open✅                          | open✅                       | open✅                       |
| close✅                         | close✅                      | close✅                      |
| get_pid✅                       | get_pid✅                    | get_pid✅                    |
| get_process_name✅              | get_process_name✅           | get_process_name✅           |
| get_process_memory_usage✅      | get_process_memory_usage✅   | get_process_memory_usage✅   |
| get_process_cpu_usage✅         | get_process_cpu_usage✅      | get_process_cpu_usage✅      |
| get_all_processes✅             | get_all_processes✅          | get_all_processes✅          |
| elevate_privileges✅            | elevate_privileges✅         | elevate_privileges✅         |
| elevate_privileges_by_pid✅     | elevate_privileges_by_pid✅  | elevate_privileges_by_pid✅  |
| **Network Interaction:**       |
| create_socket✅                 | create_socket✅              | create_socket✅              |
| listen_socket✅                 | listen_socket✅             | listen_socket✅             |
| connect_socket✅                | connect_socket✅            | connect_socket✅            |
| send_data✅                     | send_data✅                  | send_data✅                  |
| close_socket✅                  | close_socket✅               | close_socket✅               |
| is_network_available✅          | is_network_available✅       | is_network_available✅       |
| get_local_ip✅                  | get_local_ip✅               | get_local_ip✅               |
| get_external_ip✅               | get_external_ip✅            | get_external_ip✅            |
| resolve_hostname✅              | resolve_hostname✅           | resolve_hostname✅           |
| get_hostname✅                  | get_hostname✅               | get_hostname✅               |
| get_network_interfaces✅        | get_network_interfaces✅     | get_network_interfaces✅     |
| get_interface_name✅            | get_interface_name✅         | get_interface_name✅         |
|get_mac_address✅|get_mac_address✅|get_mac_address✅|
| **File System:**               |
| create_file✅                   | create_file✅                | create_file✅                |
| open_file✅                     | open_file✅                  | open_file✅                  |
| read_file✅                     | read_file✅                  | read_file✅                  |
| write_file✅                    | write_file✅                 | write_file✅                 |
| create_directory✅              | create_directory✅           | create_directory✅           |
| delete_directory✅              | delete_directory✅           | delete_directory✅           |
| move_directory✅                | move_directory✅             | move_directory✅             |
| get_directory_contents✅        | get_directory_contents✅     | get_directory_contents✅     |
| has_file_access✅               | has_file_access✅            | has_file_access✅            |
| has_directory_access✅          | has_directory_access✅       | has_directory_access✅       |
| get_file_size✅                 | get_file_size✅              | get_file_size✅              |
| get_file_creation_date✅        | get_file_creation_date✅     | get_file_creation_date✅     |
| get_file_modification_date✅    | get_file_modification_date✅ | get_file_modification_date✅ |
| get_file_owner✅                | get_file_owner✅             | get_file_owner✅             |
| create_symlink✅                | create_symlink✅             | create_symlink✅             |
| **Graphical Interface (GUI):** |
| create_window❌                 | create_window❌              | create_window❌              |
| set_window_title❌              | set_window_title❌           | set_window_title❌           |
| set_window_position❌           | set_window_position❌        | set_window_position❌        |
| set_window_size❌               | set_window_size❌            | set_window_size❌            |
| show_window❌                   | show_window❌                | show_window❌                |
| hide_window❌                   | hide_window❌                | hide_window❌                
| close_window❌                  | close_window❌               | close_window❌               |
| create_button❌                 | create_button❌              | create_button❌              |
| create_label❌                  | create_label❌               | create_label❌               |
| create_text_box❌               | create_text_box❌            | create_text_box❌            |
| create_list_box❌               | create_list_box❌            | create_list_box❌            |
| create_menu❌                   | create_menu❌                | create_menu❌                |
| register_event_handler❌        | register_event_handler❌     | register_event_handler❌     |
| handle_event❌                  | handle_event❌               | handle_event❌               |
| get_event_type❌                | get_event_type❌             | get_event_type❌             |
| get_event_data❌                | get_event_data❌             | get_event_data❌             |
| draw_line❌                     | draw_line❌                  | draw_line❌                  |
| draw_rectangle❌                | draw_rectangle❌             | draw_rectangle❌             |
| draw_circle❌                   | draw_circle❌                | draw_circle❌                |
| draw_text❌                     | draw_text❌                  | draw_text❌                  |
| draw_image❌                    | draw_image❌                 | draw_image❌                 |
| get_screen_resolution❌         | get_screen_resolution❌      | get_screen_resolution❌      |
| **Other:**                     |
| play_sound❌                    | play_sound❌                 | play_sound❌                 |
| get_key_state❌                 | get_key_state❌              | get_key_state❌              
| get_mouse_position✅            | get_mouse_position✅         | get_mouse_position✅         |
| set_timer❌                     | set_timer❌                  | set_timer❌                  |
| cancel_timer❌                  | cancel_timer❌               | cancel_timer❌               |
| alert!✅                        | alert!✅                     | alert!🕘                    |
| error!✅                        | error!✅                     | error!🕘                    |