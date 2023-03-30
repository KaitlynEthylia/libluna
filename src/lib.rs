#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "linux")]
mod linux;

mod event;
mod keys;

#[cfg(test)]
mod tests {
}
