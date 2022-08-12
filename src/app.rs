use std::time::Instant;
use tui::style::Color;
use tui::style::Style;
use tui::text::Span;

static mut LAST_LEN: usize = 0;
#[derive(Debug, Clone)]
pub struct App {
    pub text: String,
    pub misspells: Vec<bool>,
    pub index: usize,
    pub time_passed: u32,
    pub typed: u16,
    pub mistakes: u16,
    time: Instant,
    complete: bool,
}

impl App {
    pub fn new(data: String) -> App {
        App {
            misspells: vec![false; data.chars().count() - 1],
            text: data,
            index: 0,
            time: Instant::now(),
            time_passed: 0,
            complete: false,
            typed: 0,
            mistakes: 0,
        }
    }

    pub fn get_spans<'a>(&self, txt: &'a str) -> Vec<Span<'a>> {
        if txt.len() == 0 {
            return Vec::<Span>::new();
        }

        /*
        let iter = unsafe {
            if LAST_LEN < txt.chars().count() {
                txt.char_indices().skip(LAST_LEN).enumerate()
            } else {
                txt.char_indices().skip(0).enumerate()
            }
        };
        */
        unsafe {
            LAST_LEN = txt.chars().count();
        }

        fn style_span<'a>(txt: &'a str, clr: bool) -> Span {
            Span::styled(
                txt,
                Style::default()
                    .fg(Color::Black)
                    .bg(if clr { Color::Green } else { Color::Red }),
            )
        }

        let mut spans = Vec::<Span>::new();
        let mut str_pnt = 0;
        let mut end_pnt = 0;
        let mut init_val = self.misspells[0];

        for (index, _) in txt.char_indices().enumerate() {
            //If value stays the same, only make the span longer
            if init_val == self.misspells[index] {
                end_pnt += 1;
            } else {
                //Calculate start and end index of a span
                let (end_idx, _) = txt.char_indices().nth(end_pnt).unwrap();
                let (str_idx, _) = txt.char_indices().nth(str_pnt).unwrap();
                spans.push(style_span(&txt[str_idx..end_idx], init_val));

                //Only change in corectness triggers this else, so last character entered has to be
                //of the reverse color
                spans.push(style_span(
                    &txt[end_idx..end_idx + txt.char_indices().nth(end_pnt).unwrap().1.len_utf8()],
                    !init_val,
                ));

                init_val = self.misspells[index];
                end_pnt += 1;
                str_pnt = end_pnt;
            }
        }

        //If str_pnt is at start of text, do nothing
        //If it is somewhere else display rest of the text
        let str_idx = if str_pnt == 0 {
            0
        } else {
            txt.char_indices().nth(str_pnt - 1).unwrap().0
                + txt.char_indices().nth(str_pnt - 1).unwrap().1.len_utf8()
        };
        spans.push(style_span(&txt[str_idx..], init_val));

        spans
    }
    pub fn stop_timer(&mut self) {
        self.time_passed = self.time.elapsed().as_secs() as u32;
    }
    pub fn completed(&mut self) {
        self.complete = true;
    }
    pub fn is_complete(&self) -> bool {
        self.complete
    }
}
