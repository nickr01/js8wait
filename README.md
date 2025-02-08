# js8wait

A very simple utility which will exit when the system clock reaches the base of a JS8 time frame, and also play a suitable WAV file if supplied. The intention is to play a WAV file, or to allow a following script command to start at a time aligned to a JS8 frame boundary. Default would be aligned to 15sec.

Suitable WAV formats includes f32 bit with a BWF header. Recommend to use at least 16bit with 32K sample rate to exceed the requirements of js8call. This might be checked in js8wait in future.

Audio format other than WAV is discouraged because of potential to introduce confounding compression artefacts.

JS8 frame modulus or boundary is calculated from specified speeds: slow: 30 normal: 15 fast:10 turbo:6.

Lead time in a recording is automatically calculated from the reference time (samples since midnight) in BWF header of the WAV file. js8wait attempts to recreate the same realtime frame offset on playback. If the WAV file is produced by an editing process then this offset will usually be 0, and the edit process should be used to trim the audio file to be consistent with this with it's content start aligned with the js8 frame boundary.

Default lead time is 0 if no file is supplied.

Combined with calculation of current offset from wall clock frame boundary.

```
Usage: js8wait [OPTIONS]

Options:
  -f, --file <FILE>      The WAV file to use [default: none]
  -d, --device <DEVICE>  The output device to use [default: default]
  -s, --speed <SPEED>    Slowest js8speed in test. Determines time modulus
    [default: normal] [possible values: slow, normal, fast, turbo]
  -h, --help             Print help
  -V, --version          Print version
```

