# big questions to answer:

1. what is a "midi track"?
    
a sequence of midi note-on/note-off messages that is defined as running parallel to the time domain.

HOWEVER, our data representation can take advantage of the fact that silence can be represented as non-membership of the data structure, rather than needing to be explicitly tracked.

This means that our data representation for a singular track can a hashmap, where the key is the timestamp the message should be output (in milliseconds) and the value is the midi message that needs to be output.

Creating an additional midi track is appending another hashmap to our TrackList.

2. how do we handle message output?

MIDI messages are output sequentially, but our tracks _could_ have multiple simultaneous messages saved, intended to be sent out at once. For this reason, we can use a buffer to batch messages that should be sent out simulataneously. The number of messages that could be sent out at once is equal to the number of tracks we have, so the buffer size can be handled by the tape recorder itself.

3. how do virtual midi ports work?

This is going to require research into the existing libraries in Rust, and also potentially the USB MIDI documentation/reference? But if this is too tricky, we can simply set a maximum number of tracks.
