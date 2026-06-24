- [ ] Rework of `help` and `man` generation :: 
    * `man` generation is the most basic I could get away with
    - [ ] Support for separate man pages for subcommands
    - [ ] Support for example sections
    - [ ] Support for usage sections
    * `help` generation is more robust but lacking formatting if the terminal supports it
    - [ ] Most of the needed backend is already implemented in talos, but would need to be extracted, maybe into athena?
        * Talking generating valid ANSI output
    - [ ] Layout assumes single byte characters (ASCII) and needs to be updated for UTF-8
    - [ ] Expand help :: -h flag could also print a bit of general usage information
        * Grouping of flags, assigning values multiple times etc


