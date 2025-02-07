# js8wait

A very simple utility which will exit when the system clock reaches the base of a JS8 time frame, and also play a suitable WAV file if supplied. The intention is to play a WAV file, or to allow a following script command to start at a time aligned to a JS8 frame boundary. Default would be aligned to 15sec.

Suitable WAV format includes f32 with a BWF header. Recommend at least 16bit with 32K sample rate to exceed the requirements of js8call. This might be checked here in future.

Audio format other than WAV is discouraged because of potential to introduce confounding compression artefacts.

Params

JS8 frame modulus calc from specified speeds: slow:30 normal:15 fast:10 turbo:6,

Lead time autocalculated from the timestamp in BWF header of an arbitrary WAV file to use the same offset to minute boundary. If you supply such a file it will play it. Default lead time is 0 if no file.

Combined with calculation of current offset from wall clock frame boundary.

Usage: js8wait [OPTIONS]

Options:
  -f, --file <FILE>      The WAV file to use [default: none]
  -d, --device <DEVICE>  The output device to use [default: default]
  -s, --speed <SPEED>    Slowest js8speed in test. Determines time modulus
    [default: normal] [possible values: slow, normal, fast, turbo]
  -h, --help             Print help
  -V, --version          Print version
