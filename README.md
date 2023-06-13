# Printer Scanner
This is just a pretty simple and naiive approach to scanning a local network for printers.
Has a few hardcoded devices, mostly from my own school.

# Usage
- `./ scan --threads <20> --verbose false --ip_subnet "10.208.x.x" --progress_bar true --append_file false --timeout 2000`
- `./ print --ip <printer ip> --file <file to print> --copies 1 --bypass_ext false --identify_formats true --only_detect_formats false`
- Those are 2 default usages, commented parameters can be viewed [here](https://github.com/Coops0/printer-scanner/blob/master/src/main.rs#L16).
