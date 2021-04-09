extern crate crossbeam;
extern crate reqwest;
use std::io;
use std::time::Instant;

fn main() {
    loop {
        println!("Please input the number of http reqwests u desire. To exit press Ctrl+C");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("2 red, mr bomd");
        let nr_threads = match input.trim().parse::<u32>() {
            Ok(rez) => rez,
            Err(error) => {
                println!("!enuf Numbers = {}. Moar!", error);
                continue;
            }
        };
        let url_address;
        loop {
            println!("Please input the address u reqwest. To exit press Ctrl+C");
            let mut address = String::new();
            io::stdin()
                .read_line(&mut address)
                .expect("Reading address failed!");
            url_address = match reqwest::Url::parse(&address) {
                Ok(rez) => rez,
                Err(error) => {
                    println!("Error = {}. Gimme valid URL! I need it 2 maintain my saltydev xistanz! U r so hot! And salty! Example url: \"https://devbulge.mangadex.com\"",error);
                    continue;
                }
            };
            break;
        }
        let bool_req_opt_display;
        loop {
            println!("Please specify if u want reqwest sussexxx info displayed. Press Y чтобы поссать в лифе or N чтобы выпросить пиздюлей от живцов. To exit press Ctrl+C.");
            let mut req_opt_display = String::new();
            io::stdin()
                .read_line(&mut req_opt_display)
                .expect("No display ffound! Press F1 for ICE emulation");
            if req_opt_display == "Y\r\n" || req_opt_display == "y\r\n" {bool_req_opt_display=true;}
            else if req_opt_display == "N\r\n" || req_opt_display == "n\r\n" {bool_req_opt_display=false;}
            else {
                    println!("Please input a valid answer, loud and clear!");
                    continue;
            }
            break;
        }
        crossbeam::scope(|scope|{
            let mut results=(0,0,0); // (OK results, NOK results, ErrReq)
            let mut handles=Vec::new();
            let starting_time=Instant::now();
            for nr in 0..nr_threads{
                    let int_url_address=&url_address;
                    handles.push(scope.spawn(move |_|{
                    match reqwest::get( int_url_address.as_ref()){
                    Ok(response) =>{
                        match response.status() {
                            reqwest::StatusCode::OK => {
                                //println!("Status code: {}",response.status());
                                //let text = response.text().expect("Coundn't retrieve text!");
                                //println!("The text is : {}", text);
                                if bool_req_opt_display==true {println!("Request mr. {}",nr);}
                                ReqwestResult::OK
                            },
                            _ => {
                                if bool_req_opt_display==true {println!(" The scripts are not OK!");}
                                ReqwestResult::NOK
                                }
                        }
                    },
                    Err(_) => {
                        if bool_req_opt_display==true {println!("Couldn't get the response!");}
                        ReqwestResult::ErrReq
                    }
                }
                }));
            }
        for handle in handles.into_iter(){
            match handle.join().unwrap() {
                ReqwestResult::OK=>results.0=results.0+1,
                ReqwestResult::NOK=>results.1=results.1+1,
                ReqwestResult::ErrReq=>results.2=results.2+1
            }
        }
        let duration=starting_time.elapsed();
        println!("Reqwests results are : Result was 200 (OK) = {}, Result was not 200 (NOK) = {}, Reqwesting wors = {}. Total reqwests processing duration = {:?}",results.0, results.1, results.2, duration);
        }).expect("The scope didn't join threads without panicking!");
    }
}

enum ReqwestResult {
    OK,     //Result was 200
    NOK,    //Result was not 200
    ErrReq, //Error while reqwesting
}

