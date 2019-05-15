extern crate tungstenite;
extern crate xmlparser as xml;


use std::net::TcpListener;
use std::thread::spawn;
use tungstenite::server::accept;

use std::io;
use std::io::prelude::*;

use std::env;
use std::fs;

use std::io::Read;

const x3dDefault: &str = "/var/www/actop.info/html/web/files/default/default.x3d";

fn parse(text: &str) -> Result<String, String> {
	
	let mut str_ = String::new();
	
	let mut bool_ = true;
	
	for token in xml::Tokenizer::from(text) {
		match token {
			Ok(xml::Token::ElementStart { prefix, local, span }) => {
				let spanStr = span.as_str();
				if spanStr == "<X3D" {
					bool_ = false;
				} else {
					bool_ = true;
				}
				if bool_ == true {
					str_.push_str(spanStr);
				}
			}
			Ok(xml::Token::ElementEnd { end, span }) => {
				let spanStr = span.as_str();
				if  spanStr == ">" || spanStr == "/>" {
					if bool_ == true {
						str_.push_str(spanStr);
					}
				} else {
					if spanStr != "</X3D>" {
						if bool_ == true {
							str_.push_str(spanStr);
						}
					}
				}
			}
			Ok(xml::Token::Attribute { prefix, local, value, span }) => {
				if bool_ == true {
					let spanStr = span.as_str();
					str_.push_str(" ");
					str_.push_str(spanStr);
				}
			}
			_ => {
				
			}
			Err(e) => {
				let s_ = String::from("error");
				return Err(s_);
			}
		}
	}
	
	Ok(str_)
}

fn load_file(path: &str) -> Result<String, String> {
	
	let mut file = fs::File::open(path);
	
	match file {
		Ok(mut f) => {
			
			let mut text = String::new();
			let byte = f.read_to_string(&mut text);
			
			match byte {
				Ok(b) => {
					return Ok(text);
				}
				Err(e) => {
					let mut str_ = String::from("file read error ");
					str_.push_str(path);
					return Err(String::from(str_));
				}
			}
			
			
		}
		Err(e) => {
			let mut str_ = String::from("file does not exist ");
			str_.push_str(path);
			return Err(str_);
		}
	}
}

fn default_x3d() -> String {
	let text = load_file(&x3dDefault).unwrap();
	let res = parse(&text).unwrap();
	let mut str_ = String::from("-67.5 -100 0|67.5 100 5|135 200 5|0 0 2.5|4696| 
	From (0 0 321.895) 
	To (0 0 2.5 0) 
	Field of view is 45°| 
	<Viewpoint 
		id=\"front\" 
		position=\"0 0 10\" 
		orientation=\"0 0 2.5 0\" 
		description=\"camera\" 
		fieldOfView=\"0.785398\">
	</Viewpoint>|||");
	str_.push_str(&res);
	str_
}

fn main() {
	
	let server = TcpListener::bind("127.0.0.1:8080").unwrap();
	for stream in server.incoming() {
		spawn (move || {
			let mut websocket = accept(stream.unwrap()).unwrap();
			loop {
				
				let msg = websocket.read_message();
				
				match msg {
					Ok(m) => {
						if m.is_text() {
							
							let mut text = m.to_text();
							
							match text {
								Ok(txt) => {
									let text = load_file(&txt);
									match text {
										Ok(txt) => {
											let res = parse(&txt);
											
											match res {
												Ok(s) => {
													
													let mut str_ = String::from("-67.5 -100 0|67.5 100 5|135 200 5|0 0 2.5|4696| 
													From (0 0 321.895) 
													To (0 0 2.5 0) 
													Field of view is 45°| 
													<Viewpoint 
														id=\"front\" 
														position=\"0 0 321.895\" 
														orientation=\"0 0 2.5 0\" 
														description=\"camera\" 
														fieldOfView=\"0.785398\">
													</Viewpoint>|||");
													str_.push_str(&s);
													websocket.write_message(tungstenite::Message::Text(str_)).unwrap();
													
												}
												Err(e) => {
													let str_ = default_x3d();
													websocket.write_message(tungstenite::Message::Text(str_)).unwrap();
												}
											}
										}
										Err(e) => {
											let str_ = default_x3d();
											websocket.write_message(tungstenite::Message::Text(str_)).unwrap();
										}
									}
								}
								Err(e) => {
									let str_ = default_x3d();
									websocket.write_message(tungstenite::Message::Text(str_)).unwrap();
								}
							}
						}
					}
					Err(e) => {
						break;
					}
				}
			}
		});
	}
}
