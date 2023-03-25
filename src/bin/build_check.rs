//This is simple build error checker for other projects.
//Idea is to read build ouptuts and conclude how to "heal" the project.

use std::process::Command;

fn main() {
    let output = Command::new("cargo")
        .args(&["check"]) //and after check has no ERRORs , "build --release"
        .output()
        .expect("Failed to build project");

    if output.status.success() {
        println!("Project built successfully!");
    } else {
        /*
            If the command failed, you can parse the stderr output of the command to find out what went wrong.
            The stderr output is stored in the stderr field of the std::process::Output struct returned by the output() method.
            You can convert the stderr output to a string using the String::from_utf8_lossy() method, like this:
         */
        let error_output = String::from_utf8_lossy(&output.stderr);

        println!(
            "Failed to build project: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}
//EXAMPLE CODE> 
//              CHECK ERROR OUTPUTS AND ONLY PARSE Those + Line bellow it 

// let mut prev_line: Option<&str> = None;
// for (i, line) in error_output.lines().enumerate() {
//     if line.contains("error") {
//         // Print the error message
//         println!("{}", line);

//         // Print the next line if it exists and doesn't contain "error"
//         if let Some(next_line) = error_output.lines().nth(i+1) {
//             if !next_line.contains("error") {
//                 println!("{}", next_line);
//             }
//         }

//         // Set the previous line to None so that we don't print it again
//         prev_line = None;
//     } else if prev_line.is_some() {
//         // If the previous line contained "error", print this line as well
//         println!("{}", line);
//     }
//     prev_line = Some(line);
// }
/*

This code creates a new Command instance that runs the cargo build --release command,
then waits for the command to finish and returns the output of the command as a std::process::Output struct.
If the command succeeds, the code prints a success message, otherwise it prints the stderr output of the command.

Run the build.rs file by running the cargo build command. This will run the build.rs file as a build script before building your project.
Note that the std::process::Command crate can also be used to run other commands,
 not just build commands. You can use it to run any command-line program and capture its output. */
