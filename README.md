# js8wait

A very simple utility which will exit when the system clock reaches the base of a JS8 time frame.

The intention is to allow a following script command to start at a time aligned to a JS8 frame boundary.

Default would be 15sec, no offset.

Intended params:

JS8 frame modulus calc from speeds: /S:30 /N:15 /F:? /T:?

Manual Offset in +-0.1 secs

Auto offset calc from timestamp in BWF header of an arbitrary WAV file. Default 0 if no file.

Manual and Auto offsets are additive to allow compensation for example for start delay in command line player that might follow.
