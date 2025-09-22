use chrono::Local;

pub fn get_header () -> &'static str {
  return concat!(
    "   ▄████████    ▄████████    ▄████████    ▄████████    ▄████████    ▄████████ ████████▄  \n",
    "  ███    ███   ███    ███   ███    ███   ███    ███   ███    ███   ███    ███ ███   ▀███ \n",
    "  ███    ███   ███    █▀    ███    █▀    ███    █▀    ███    █▀    ███    █▀  ███    ███ \n",
    " ▄███▄▄▄▄██▀   ███          ███         ▄███▄▄▄      ▄███▄▄▄      ▄███▄▄▄     ███    ███ \n",
    "▀▀███▀▀▀▀▀   ▀███████████ ▀███████████ ▀▀███▀▀▀     ▀▀███▀▀▀     ▀▀███▀▀▀     ███    ███ \n",
    "▀███████████          ███          ███   ███          ███    █▄    ███    █▄  ███    ███ \n",
    "  ███    ███    ▄█    ███    ▄█    ███   ███          ███    ███   ███    ███ ███   ▄███ \n",
    "  ███    ███  ▄████████▀   ▄████████▀    ███          ██████████   ██████████ ████████▀  \n",
    "  ███    ███                                                                             \n",
    "\n",
    "Subscribe to RSS-Feeds and share them!\n"
  );
}

pub fn get_log_timestamp () -> String {
  Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn log_info (data: impl Into<String>) {
  println!("[INFO] {} @ {}", data.into(), get_log_timestamp());
}

pub fn log_error (data: impl Into<String>) {
  eprintln!("[ERROR] {} @ {}", data.into(), get_log_timestamp());
}

pub fn log_warning (data: impl Into<String>) {
  println!("[WARNING] {} @ {}", data.into(), get_log_timestamp());
}