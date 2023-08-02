# Mail-Client-CLI-RUST
 This is a Rust command-line application that allows you to send and receive emails using the SMTP and IMAP protocols respectively. It utilizes the lettre, tokio, and native_tls crates for email functionality, and the imap and native_tls crates for IMAP functionality.
# Features
1. **Send Email:** You can send an email by providing the sender's email address, recipient's email address, subject, and body of the email. The application will use the provided SMTP server details to send the email.

2. **Receive Email:** You can receive emails using IMAP by providing your IMAP username and password. The application connects to the IMAP server and fetches the latest email from the Inbox. The received email body is printed to the console and saved in a mails.txt file.
# Prerequisites
You need to have Rust and Cargo installed on your system. You can download them from the official https://www.rust-lang.org
# Installation
1. Clone this repository to your local machine:
  
- git clone https://github.com/Anurag-Mohan/Mail-Client-CLI-RUST.git
- cd Mail-Client-CLI-RUST
  
2. Build the application using Cargo:
- cargo build --release
# Usage
1. Run the application: cargo run --release
2. The application will display a menu with three options:
- Enter 1 to send an email.
- Enter 2 to receive an email.
- Enter 3 to exit the program.
3. If you choose to send an email (option '1'), follow the prompts to input the sender's email, recipient's email, subject, and body of the email. The email will be sent using the provided SMTP server details.
4. If you choose to receive an email (option '2'), you'll be prompted to enter your IMAP username and password. The application will connect to the IMAP server, fetch the latest email from the Inbox, and display its body on the console. Additionally, the email body will be saved to a 'mails.txt' file.
5. To exit the program (option '3'), the application will display a message and terminate.
# Configuration
- ***SMTP Server:*** Update the domain, port, username, and password variables in the code to match your SMTP server details.
  If you are using gmail's smtp server you need to enable the two factor authentication and generate an app password. That password should be given in the password part
- ***IMAP Server:*** Update the domain variable in the receive_mail function to match your IMAP server details.
  If you are using using gmail's imap server you have to enable imap in gmail settings
# Dependencies
- [`lettre`](https://crates.io/crates/lettre): A library for sending and receiving emails.
- [`tokio`](https://crates.io/crates/tokio): An asynchronous runtime for Rust.
- [`native_tls`](https://crates.io/crates/native_tls): A crate for TLS/SSL functionality.
- [`imap`](https://crates.io/crates/imap): A library for working with the IMAP protocol.

# Contributions 
We welcome and appreciate contributions to make this project better! Whether you're an experienced developer or just getting started, there are many ways you can contribute.
### How to Contribute
1. ***Fork the Repository:*** Start by forking this repository to your own GitHub account.
2. ***Clone the Repository:*** Clone the forked repository to your local machine using.
3. ***Create a Branch:*** Create a new branch for your contribution.
4. ***Make Changes:*** Make your desired changes to the codebase. Feel free to add new features, fix bugs, improve documentation, or anything else that adds value.
5. ***Commit and Push:*** Commit your changes and push them to your forked repository
6. ***Open a Pull Request:*** Go to your forked repository on GitHub and open a pull request. Provide a clear description of your changes and why they're valuable.
### Guidelines
- Follow the existing code style and naming conventions.
- Write clear and concise commit messages.
- Test your changes thoroughly before submitting a pull request.
##  Screenshot
<table>
 <tr>
  <th>
   <img src="">
  </th>
 </tr>
</table>

