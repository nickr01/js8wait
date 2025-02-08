# js8wait

A very simple utility which will exit when the system clock reaches the base of a JS8 time frame, and also play a suitable WAV file if supplied. The intention is to play a WAV file, or to allow a following script command to start at a time aligned to a JS8 frame boundary. Default would be aligned to 15sec.

Suitable WAV formats includes f32 bit with a BWF header. Recommend to use at least 16bit with 32K sample rate to exceed the requirements of js8call. This might be checked in js8wait in future.

Audio format other than WAV is discouraged because of potential to introduce confounding compression artefacts.

JS8 frame modulus or boundary is calculated from specified speeds: slow: 30 normal: 15 fast:10 turbo:6.

Lead time in a recording is automatically calculated from the source origation time in BWF header of the WAV file. js8wait attempts to recreate the same realtime frame offset on playback.

Default lead time is 0 if no file is supplied.

js8wait combines the lead time with realtime offset from wall clock frame boundary to then delays the play start to the correct time.

### Workflow

Produce a WAV file from recording software eg Audacity or Reaper which directly record a WAV files. Reaper saves raw recorded tracks in a Media directory and they are perfect to feed to js8wait as they have the origination time embedded, using these files is very reliable.

TODO ** check a workflow for Audacity.

If the WAV file is being produced as the result of a DAW editing process then the origination_time is set to that of the start of the render process and this will be misleading.

The workaround is to slide the test signal to the right to align it frames with js8 frames, and to start the render range  at any point other than 0. Time_reference will now be set to the correct offset, and being non-zero will take priority in determining the lead in time calculation. Tested with Reaper. TODO ** test with Audacity


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

