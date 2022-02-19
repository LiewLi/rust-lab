use anyhow::Result;
use std::fs;
use std::fs::metadata;

use super::playlist::PlayList;

fn parse_play_list(url: &str) -> Result<PlayList> {
    let md = metadata(url)?;
    if md.is_dir() {
        let paths = fs::read_dir(url)?;
        let play_list = paths
            .filter_map(|p| p.ok())
            .filter_map(|path| path.path().into_os_string().into_string().ok())
            .filter(|p| p.ends_with(".mp3"))
            .collect();

        Ok(PlayList::new(play_list))
    } else if md.is_file() {
        let play_list = vec![url]
            .iter()
            .filter(|p| p.ends_with(".mp3"))
            .map(|p| p.to_string())
            .collect();
        Ok(PlayList::new(play_list))
    } else {
        Ok(PlayList::new(vec![]))
    }
}

pub fn play_file(url: &str) -> Result<()> {
    let mut play_list = parse_play_list(url)?;

    if play_list.urls.is_empty() {
        println!("music file not found");
    } else {
        loop {
            println!("{}", play_list);
            play::play(&play_list.urls[play_list.cur_index])?;
            play_list.cur_index = (play_list.cur_index + 1) % play_list.urls.len();
        }
    }

    Ok(())
}
