pub mod text{
    use crossterm::event::{KeyCode, KeyEvent};
use uuid::Uuid;


    pub struct Chat{
        id: Uuid,
        history: Vec<String>,
        cur_buff: String,
        crsor_pos: usize,
        caps_lck: bool,
    }

    impl Chat{
        pub fn new() -> Self{
            Self { id: Uuid::new_v4(), history: vec![], cur_buff: "".to_string(),crsor_pos:0,caps_lck:false}
        }

        pub fn get_history(&self) -> Vec<String>{
            self.history.clone()
        }

        pub fn get_last_message(&self) -> Option<&String>{
            self.history.last()
        }

        pub fn handle(&mut self,key: KeyEvent){

            match key.code{
                KeyCode::Enter => {
                    self.history.push(self.cur_buff.clone());
                    self.cur_buff.clear();
                    self.crsor_pos = 0;
                },
                KeyCode::Backspace => {
                    if self.cur_buff.len() >= 1{
                        self.cur_buff.pop().unwrap();
                    }
                    self.crsor_pos = self.crsor_pos.saturating_sub(1);
                },
                KeyCode::Char(ch) => {
                    if self.caps_lck{
                        self.cur_buff.push(ch.to_ascii_uppercase());
                    }else{
                        self.cur_buff.push(ch);
                    }
                    self.crsor_pos = self.crsor_pos.saturating_add(1);
                },
                KeyCode::Left => {
                    self.crsor_pos = self.crsor_pos.saturating_sub(1);
                },
                KeyCode::Right => {
                    self.crsor_pos = self.crsor_pos.saturating_add(1);
                },
                KeyCode::Home => {
                    self.crsor_pos = 0;
                },
                KeyCode::End => {
                    self.crsor_pos = self.cur_buff.len();
                },
                KeyCode::CapsLock => {
                    self.caps_lck = if self.caps_lck {false}else {true};
                },
                KeyCode::Delete => {
                    if self.cur_buff.len() >= 1{
                        self.cur_buff.pop().unwrap();
                    }
                    self.crsor_pos = self.crsor_pos.saturating_sub(1);
                },
                _ => {}
            }

        }



    }





}