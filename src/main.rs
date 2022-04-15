use std::fs;
use std::string::String;
use std::env;
use std::fs::File;
use std::io::Write;

fn main() {
    let args: Vec<String> = env::args().collect();
    //println!("{:?}", args);

    if args.len() < 4{
        println!("Not enough command line arguments!");
        println!("generator.exe [masterpages directory] [content page directory] [output path directory]");
        println!("Example: generator.exe ./drafts/masterpages ./drafts/pages ./html");
        return;
    }

    let masterpage_path = &args[1];
    let content_path = &args[2];
    let output_path = &args[3];

    process_pages(content_path,masterpage_path,output_path);
}

fn process_pages(content_path: &str, masterpage_path: &str, output_path: &str){
    for file in fs::read_dir(content_path).unwrap(){
        let path_buf = file.unwrap().path();
        let path = path_buf.to_str().unwrap();
        process_page(&path,masterpage_path,output_path);
    }
}

fn process_page(path: &str, masterpage_path: &str, output_path: &str){
    let mut content = String::new();
    read_file(path, &mut content);
    
    let clean_path = path.replace('\\',"/");
    let name = clean_path.rsplit_once('/').unwrap().1;

    let first_line_end = content.find('\n').unwrap();
    let mut header = String::from(content.get(0..first_line_end).unwrap());
    // remove \r on windows line endings
    if header.ends_with('\r'){
        header.remove(header.len()-1);
    }
    //check if tagged correctly
    if header.starts_with('#'){
        header.remove(0);
    }
    else{
        println!("Page {:?} has no valid header!",path);
        return;
    }

    // get the masterpage name
    let elements : Vec<_> = header.split('/').collect();    
    let masterpage = elements[0];
    let marker = elements[1];

    let mut template_html = String::new();
    if read_masterpage(masterpage,masterpage_path, &mut template_html){
        println!("Placing {:?} at {:?} -> {:?}...",path,masterpage,marker);
        let search_string = format!("#!{}!#",marker);
        
        let mut page_html = String::new();
        read_file(path, &mut page_html);

        let skip_chars = page_html.find('\n').unwrap();
        let writeable = page_html.get(skip_chars..).unwrap();
        
        let result : String = template_html.replace(search_string.as_str(), &writeable);
        if result.eq(&template_html){
            println!("Keyword {:?} not found in masterpage",search_string);
        }else{
            //println!("{:?}",result);
            let output_file_path = format!("{}/{}",output_path,name);
            println!("Exporting to {:?}",output_file_path);
            save_text(&output_file_path, &result);
        }
    }
    else{
        println!("Failed to process page {:?}",path);
    }
}

fn save_text(path: &str, content : &String){
    let full_path = std::path::Path::new(path);
    let prefix = full_path.parent().unwrap();
    fs::create_dir_all(prefix).unwrap();

    let mut file = File::create(path).unwrap();
    writeln!(&mut file, "{}",content).unwrap();
}

/*
fn list_templates(path: &str){
    for file in fs::read_dir(path).unwrap() {
        let path = file.unwrap().path();
        let name = path.file_name().unwrap().to_str().unwrap().strip_suffix(".html").unwrap(); //rust is weird
        println!("{:?}",name);
    }
}
*/

fn read_masterpage(name: &str, path: &str, html: &mut String) -> bool {
    let path = format!("{}/{}.html",path,name);
    let exists = std::path::Path::new(&path).exists();
    if exists{
        read_file(&path,html);
        return true;
    }
        println!("Masterpage {} doesn't exist!",name);
        return false;
    
}

fn read_file(path: &str, content: &mut String) {
    let s = fs::read_to_string(path)
    .expect("Something went wrong reading the file");
    content.push_str(&s);
}