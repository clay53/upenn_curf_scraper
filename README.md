# UPenn CURF Scraper
This software is for downloading the [Center for Undergraduate Research and Funding Research Directory at the University of Pennsylvania](https://curf.upenn.edu/undergraduate-research/research-directory) so that it can be searched easier.

## Usage
This program was made for utility, not user experience. Because of that, the easiest way to create flexible custom search filters was to just [hard-code a function](src/filter.rs). As a result, if you want to change the search function, you must compile it yourself. Lookup how to set up a Rust programming environment and remember that this project uses nightly Rust so before running something like `cargo run --release -- update-auth`, you must first run `rustup override set nightly`. If you have [Nix](https://nix.dev/tutorials/install-nix) installed you can enter a complete environment by just running `nix-shell`.

## Stability Notice
There is no standard API for accessing the CURF research directory so this software depends on the layout of the website being consistent. Consequently, this software will probably break every time the Research Directory webpage changes. The original developer ([Clayton Hickey](https://claytonhickey.me)) is probably not that concerned about it breaking. If you need someone to fix it immediately, pay them or fix it yourself. Otherwise, just leave an issue in the issue tracker with all the information you can think of and more that would help someone fix the issue, but make sure no else has reported it already.

## Notice
The developer(s) do not take liability for misuse of this software. Please avoid scraping as much as possible. Do not share scraped data. If used responsibly, this software is both more convenient for the user and reduces overall load (though may cause short peaks) on the CURF servers.