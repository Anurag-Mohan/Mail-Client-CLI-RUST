use std::io::{self, Write};
use lettre::Transport;
use lettre_email::Email;
use tokio::runtime::Runtime;
use native_tls::{TlsConnector, Protocol};
use lettre::smtp::authentication::{Credentials, Mechanism};
use lettre::ClientTlsParameters;
use lettre::SmtpClient;
use std::fs::File;






async fn send_mail(sender: &str, recipient: &str, subject: &str, body: &str, creds: &Credentials) -> Result<(), Box<dyn std::error::Error>> {
 
    // Create an email message
    let email = Email::builder()
        .from(sender)
        .to(recipient)
        .subject(subject)
        .text(body)
        .build()
        .unwrap();

    let domain = "smtp.gmail.com"; // Update with your SMTP server domain
    let port = 465; // Update with your SMTP server port
    let username = "example@gmail.com"; // Update with your SMTP username
    let password = "password"; // Update with your SMTP password
    let tls_parameters_connector = TlsConnector::builder()
        .min_protocol_version(Some(Protocol::Tlsv11))
        .build()
        .unwrap();

    // Connect to the SMTP server
    let tls_parameters_client = ClientTlsParameters::new(domain.to_string(), native_tls::TlsConnector::new()?);
    let mailer = SmtpClient::new((domain, port), lettre::ClientSecurity::Wrapper(tls_parameters_client))?;

    // Authenticate to the SMTP server
    let creds = Credentials::new(username.to_string(), password.to_string());
    let mailer = mailer.credentials(creds).authentication_mechanism(Mechanism::Login);

    // Send the email
   
    let mut transport = mailer.transport();
    transport.send(email.into())?;


    Ok(())
}
extern crate imap;
extern crate native_tls;
async fn receive_mail(username: &str, password: &str) -> imap::error::Result<Option<String>> {
  let domain = "imap.gmail.com"; // Update with your IMAP server domain
   

    let tls= native_tls::TlsConnector::builder().build().unwrap();
    let client= imap::connect((domain, 993), domain, &tls)?;

   
    let mut imap_session = client.login(username, password).map_err(|e| e.0)?;//.await

    // Select the mailbox (Inbox in this case)
    
    imap_session.select("INBOX")?;

    // Fetch the latest email
    let messages = imap_session.fetch("1", "RFC822")?;
    let message= if let Some(m)= messages.iter().next() {m} 
       
     else {
       return Ok(None);
    };
if let Some(body) = message.body(){
let body_mail =std::str::from_utf8(body)
.expect("message was valid utf-8")
.to_string();
    Ok(Some(body_mail))}
    else {
        Ok(None)
    }
}

fn main() {
    // Get user input for email details
    println!("Enter IMAP username:");
    let mut username = String::new();
    io::stdin().read_line(&mut username).expect("Failed to read input");
    let username = username.trim();

    println!("Enter IMAP password:");
    let mut password = String::new();
    io::stdin().read_line(&mut password).expect("Failed to read input");
    let password = password.trim();

     let rt = Runtime::new().unwrap();
   

    loop {
        println!("Menu:");
        println!("1. Send Email");
        println!("2. Receive Email");
        println!("3. Exit");
        println!("Enter your choice:");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");

        match choice.trim().parse::<u32>() {
            Ok(1) => {

    println!("Enter sender email:");
    let mut sender = String::new();
    io::stdin().read_line(&mut sender).expect("Failed to read input");
    let sender = sender.trim();

    println!("Enter recipient email:");
    let mut recipient = String::new();
    io::stdin().read_line(&mut recipient).expect("Failed to read input");
    let recipient = recipient.trim();

    println!("Enter email subject:");
    let mut subject = String::new();
    io::stdin().read_line(&mut subject).expect("Failed to read input");
    let subject = subject.trim();

    println!("Enter email body:");
    let mut body = String::new();
    io::stdin().read_line(&mut body).expect("Failed to read input");
    let body = body.trim();

    
    let creds = Credentials::new(username.to_string(), password.to_string());
   
    rt.block_on(async {
        // Send the email
        match send_mail(sender, recipient, subject, body, &creds).await {
            Ok(_) => println!("Email sent successfully."),
            Err(e) => eprintln!("Error sending mail: {:?}", e),
        }
    })
            }
            Ok(2) => {
                // Code to receive email
              
                rt.block_on(async {
                    match receive_mail(username, password).await 
                   {  
                        Ok(body_mail) => {
                           
                                println!("Email received");
                                println!("{:?}",body_mail);
                            
                            let mut file =File::create("mails.txt").unwrap();
                               writeln!(file,"{:?}",body_mail).unwrap();
                              
                            }
                          
                        
                
                
                        Err(e) => eprintln!("Error receiving mail: {:?}", e),
                    }
                })
            }
            
Ok(3) => {
    // Exit the program
    println!("Exiting...");
    break;
}
_ => {
    println!("Invalid choice. Please select a valid option (1, 2, or 3).");
}
}
}
}


