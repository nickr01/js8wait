# js8wait

A very simple utility which will exit when the system clock reaches the base of a JS8 time frame, and also play a suitable WAV file if supplied. The intention is to play a WAV file, or to allow a following script command to start at a time aligned to a JS8 frame boundary. Default would be aligned to 15sec.

Suitable WAV formats includes f32 bit with a BWF header. Recommend to use at least 16bit with 32K sample rate to exceed the requirements of js8call. This might be checked in js8wait in future.

Audio format other than WAV is discouraged because of potential to introduce confounding compression artefacts.

JS8 frame modulus or boundary is calculated from specified speeds: slow: 30 normal: 15 fast:10 turbo:6.

Lead time in a recording is automatically calculated from the source origation time in BWF header of the WAV file. js8wait attempts to recreate the same realtime frame offset on playback.

Default lead time is 0 if no file is supplied.

### Worflow

Produce a WAV file from recording software eg Audacity or Reaper which directly record a WAV files. Reaper saves raw recorded tracks in a Media directory and they are perfect to feed to js8wait as they have the origination time embedded, using these files is very reliable.

TODO ** check a workflow for Audacity.

TODO ** check a workflow if the WAV file is produced as the result of an editing process. Ideally would trim so starts on a js8 frame boundary, with embedded origin time of 00:00:00 but need to check that the DAWs are not putting the render start time in that field.

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

