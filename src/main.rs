//!
//! Application entrypoint.
//!

/// macro: print info text.
#[macro_export]
macro_rules! info {
	() => {
		println!("");
	};
	($($arg:tt)*) => {
		println!("[INFO] {}", format!($($arg)*));
    };
}

/// macro: print error text.
#[macro_export]
macro_rules! error {
	() => {
		println!("");
	};
	($($arg:tt)*) => {
		println!("[ERROR] {}", format!($($arg)*));
	};
}

/// Capture by regexpression matching.
fn matches(string_value: &str, expression: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
	let expression = regex::Regex::new(&expression)?;

	// try to capture by "(...)".
	let capture_result = expression.captures(&string_value);
	if capture_result.is_none() {
		info!("NOT MATCHED for expression [{}].", expression);
		return Ok(Vec::new());
	}

	info!("MATCHED for expression [{}].", expression);

	// capture result
	let capture_result = capture_result.unwrap();

	let mut result: Vec<String> = vec![];

	let mut index = 0;

	for e in capture_result.iter() {
		if index == 0 {
			// Skip the first element that is not a capture.
			index += 1;
			continue;
		}
		let matched = e.unwrap();
		let string = matched.as_str().to_string();
		result.push(string.to_string());
		index += 1;
	}

	return Ok(result);
}

/// Increment build number in version string. (0.0.1 >> 0.0.2)
///
/// # Arguments
/// * `version` - Version string. (#.#.#)
fn increment_build_number(version: &str) -> Result<String, Box<dyn std::error::Error>> {
	let result = matches(version, r#"(\d+)\.(\d+)\.(\d+)"#)?;
	if result.len() != 3 {
		return Ok(version.to_string());
	}

	let left = result[0].clone();

	let middle = result[1].clone();

	let right = result[2].clone();
	let right = right.parse::<u32>()?;
	let right = right + 1;

	let result = format!("{}.{}.{}", left, middle, right);

	return Ok(result);
}

/// Get version string in text. "#.#.#"
fn read_version_string(line: &str) -> Result<String, Box<dyn std::error::Error>> {
	// Check the line.
	if !is_version_line(line) {
		return Ok("".to_string());
	}

	// Matching the version string.
	let result = matches(line, r#"\s*version\s*=\s*"(.*)""#)?;
	if result.len() != 1 {
		return Ok("".to_string());
	}
	let version_string = result[0].clone();
	if version_string == "" {
		return Ok("".to_string());
	}

	return Ok(version_string);
}

/// Check if the line is a version line.
fn is_version_line(line: &str) -> bool {
	return line.trim().starts_with("version");
}

/// Convert string to quoted string.
fn quoted(s: &str) -> String {
	return format!("\"{}\"", s);
}

/// Carefully replace version string in text.
///
/// # Arguments
/// * `line` - Line text.
/// * `version` - Version string. (#.#.#)
/// * `new_version` - New version string. (#.#.#)
fn replace_string_carefully(line: &str, version: &str, new_version: &str) -> Result<String, Box<dyn std::error::Error>> {
	let placeholder = quoted(&version);
	let new_version = quoted(&new_version);
	let result_string = line.replace(&placeholder, &new_version);
	return Ok(result_string);
}

/// Convert version string.
fn update_version_string_if_needed(line: &str, new_version: &str) -> Result<String, Box<dyn std::error::Error>> {
	// Detect version "#.#.#" string.
	let version = read_version_string(line)?;
	if version == "" {
		// No version string.
		return Ok(line.to_string());
	}

	// Replace version number carefully.
	let converted_line = replace_string_carefully(line, &version, &new_version)?;

	info!("AFFECTED LINE:\n        SRC [{}]\n        NEW [{}]", line, &converted_line);

	return Ok(converted_line);
}

/// Detect version from file.
fn detect_version_from_file(path: &str) -> Result<String, Box<dyn std::error::Error>> {
	// Read file content.
	let text = std::fs::read_to_string(path)?;

	// Convert version line.
	let lines = text.lines();
	for line in lines {
		let version = read_version_string(line)?;
		if version != "" {
			return Ok(version);
		}
	}

	return Ok("".to_string());
}

/// Increment cargo version.
fn update_cargo_version(path: &str, version: &str) -> Result<(), Box<dyn std::error::Error>> {
	// Read file content.
	let text = std::fs::read_to_string(path)?;

	// Convert version line.
	let lines = text.lines();
	let mut result_lines: Vec<String> = vec![];
	for line in lines {
		let line = update_version_string_if_needed(line, version)?;
		result_lines.push(line);
	}
	let content = result_lines.join("\n") + "\n";

	// Write file content.
	std::fs::write(path, content)?;

	return Ok(());
}

struct Application;

impl Application {
	pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
		// Detect version from Cargo.toml.
		let version = detect_version_from_file("Cargo.toml")?;

		// Increment build number. (3rd field)
		let new_version = increment_build_number(&version)?;

		// Update version in Cargo.toml.
		update_cargo_version("Cargo.toml", &new_version)?;

		// Update version in Cargo.lock.
		update_cargo_version("Cargo.lock", &new_version)?;

		return Ok(());
	}
}

fn main() {
	let app = Application {};
	let result = app.run();
	if result.is_err() {
		error!("{}", result.err().unwrap());
		std::process::exit(1);
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_increment_build_number() {
		let result = increment_build_number("0.0.0").unwrap_or_default();
		assert_eq!(result, "0.0.1");

		let result = increment_build_number("0.0.1").unwrap_or_default();
		assert_eq!(result, "0.0.2");

		let result = increment_build_number("0.0.9").unwrap_or_default();
		assert_eq!(result, "0.0.10");

		let result = increment_build_number("0.0.10").unwrap_or_default();
		assert_eq!(result, "0.0.11");

		let result = increment_build_number("0.0.99").unwrap_or_default();
		assert_eq!(result, "0.0.100");

		let result = increment_build_number("0.0.100").unwrap_or_default();
		assert_eq!(result, "0.0.101");

		let result = increment_build_number("0.0.999").unwrap_or_default();
		assert_eq!(result, "0.0.1000");

		let result = increment_build_number("0.0.1000").unwrap_or_default();
		assert_eq!(result, "0.0.1001");

		let result = increment_build_number("0.0.9999").unwrap_or_default();
		assert_eq!(result, "0.0.10000");

		let result = increment_build_number("0.0.10000").unwrap_or_default();
		assert_eq!(result, "0.0.10001");

		let result = increment_build_number("0.0.99999").unwrap_or_default();
		assert_eq!(result, "0.0.100000");

		let result = increment_build_number("0.0.100000").unwrap_or_default();
		assert_eq!(result, "0.0.100001");

		let result = increment_build_number("0.0.999999").unwrap_or_default();
		assert_eq!(result, "0.0.1000000");

		let result = increment_build_number("0.0.1000000").unwrap_or_default();
		assert_eq!(result, "0.0.1000001");

		let result = increment_build_number("0.0.9999999").unwrap_or_default();
		assert_eq!(result, "0.0.10000000");

		let result = increment_build_number("0.0.10000000").unwrap_or_default();
		assert_eq!(result, "0.0.10000001");
	}
}
