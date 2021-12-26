pub enum Message {
    ErrorMsg,
    CreateMsg,
    FoundMsg,
    WarningMsg,
}

impl Message {
    pub fn print(msg: Message, text: &str) -> () {
        let black = "\x1b[30;1m";
        let cyan = "\x1b[36;1m";
        let dim = "\x1b[2m";
        let reset = "\x1b[0m";
        let bg_red = "\x1b[41m";
        let bg_green = "\x1b[42m";
        let bg_yellow = "\x1b[43m";
        let green = "\x1b[32m";

        let arrow = format!(" {}->{}", dim, reset);
        let info = format!("{}{}{}", cyan, text, reset);
        let ok = format!("{}{} OK! {}", bg_green, black, reset);
        let failed = format!("{}{} ERROR! {}", bg_red, black, reset);
        let warning = format!("{}{} WARN {}", bg_yellow, black, reset);
        let success = format!("{}{}{}", green, text, reset);

        let message = match msg {
            Message::CreateMsg => format!(" {} File {} created!\n", ok, success),
            Message::ErrorMsg => format!(" {} {}\n", failed, text),
            Message::FoundMsg => format!(" {} A {} file was found.\n", arrow, info),
            Message::WarningMsg => format!(" {} {}\n", warning, text),
        };

        eprintln!("{}", message);
    }
}

pub fn help() {
    let bold = "\x1b[1m";
    let reset = "\x1b[0m";

    println!(
        r"Reator: Simple React CLI for React Projects

{} 
    reator <command> <template> <path> [options]

{}
    n | new | create         Creates a new file
    h | help                 Shows this screen

{} 
    rc | component           React Component
    rc | native              React Component
    ct | context             React Context API file
    np | next-page           Next.js Page
    nd | next-doc            Next.js '_document' file
    s  | style               CSS Module
    sc | styled              Styled Component

{} 
    Filename only, no extension needed. 
    Reator creates folders and files automatically.

{}
    --ws / --with-jstyle     Creates a component in a folder, with a style.js    
    --css                    Creates a component in a folder, with a CSS Module  
    --sass                   Creates a component in a folder, with a Sass Module 
    --i / --iprops           Creates a component with a 'Props' interface        
    --t / --tprops           Creates a component with a 'Props' type             
    ",
        format!("{}Usage:{}", bold, reset),
        format!("{}Commands:{}", bold, reset),
        format!("{}Templates:{}", bold, reset),
        format!("{}Path:{}", bold, reset),
        format!("{}Options:{}", bold, reset),
    )
}
