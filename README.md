# js8wait

A very simple utility which will exit when the system clock reaches the base of a JS8 time frame.

The intention is to allow a following script command to start at a time aligned to a JS8 frame boundary.

Default would be 15sec, no offset.

Intended params:

JS8 frame modulus calc from specified speeds: /S:30 /N:15 /F:10 /T:6, or autocalculated from the timestamp in BWF header of an arbitrary WAV file to use the same offset to minute boundary. Default 0 if no file.

Manual Offset in +-0.1 secs

Manual and Auto offsets are additive to allow compensation for example for start delay in command line player that might follow.
