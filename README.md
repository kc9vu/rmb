# remove bytes (rmb)

```
Usage:
  rmb <OFFSET> <INPUT> <OUTPUT>
    Reads all but the first <OFFSET> bytes from the <INPUT> and writes them to
    the <OUTPUT>, with a buffer of 4 kB.
    <OFFSET> can be negative, then only the last <OFFSET>(absolute value) bytes.
    <INPUT> and <OUTPUT> cannot be the same (case insensitive, although sometimes
    they are different files)
  rmb h[elp]
    Show help
  rmb v[ersion]
    Show version
```
