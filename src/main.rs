//Sequential
//According to the consol this too ~10 seconds to compile and run.
//
//
//
//
//

use std::error::Error;

fn count_word_occurrences(urls: &[&str]) -> Result<Vec<usize>, Box<dyn Error>> {
    let mut the_counts = Vec::new(); // counts the amount of "the"s

    for url in urls {
        let response = ureq::get(url).call()?; //  request info from the URL
        let text = response.into_string()?; //convert request to string

        // turn to individual words
        let words: Vec<&str> = text.split_whitespace().collect();

        // Count occurrences of the word "the"
        let mut the_count = 0;
        for word in &words {
            if *word == "the" {
                the_count += 1;
            }
        }

        the_counts.push(the_count);
    }

    Ok(the_counts) // Return vector containing word counts
}

fn main() {
  //urls
    let urls = vec![
        "https://www.gutenberg.org/cache/epub/325/pg325-images.html",
        "https://www.gutenberg.org/cache/epub/11417/pg11417-images.html",
        "https://www.gutenberg.org/cache/epub/20/pg20-images.html",
    ];
//this is the function call
    match count_word_occurrences(&urls) {
        Ok(the_counts) => {
            for (index, count) in the_counts.iter().enumerate() {
              //print the result
                println!("Occurrences of 'the' in file {}: {}", index + 1, count);
            }
        }
        Err(err) => {
            eprintln!("Error: {}", err);
        }
    }
}