# M-track: a tape-track inspired midi sequencer

## summary:

first crack with Rust, using Slint.

Gonna think about the architecture later, but here are a few requirements I've thought through.

1. infinite number of tracks, use a rotary encoder to go through them. This could be changed/configured in software, because restriction breeds creativity.
2. each track consists of a stream of "track messages", each of which has a midi message embedded in it, *paired* with an **output** parameter, which indicates which midi output it's related to.
    - there might be a limitation on the number of outputs that are allowable here, because each one will become a different "midi controller" at the web interface level, and need to be mapped to a different synthesizer.
    - a "track" is just a sequence of note-on and note-off messages, destined for a variety of outputs. But what gets trickier is when we use our in/out points to cut perhaps multiple midi messages short. That's when we need to _move_ note-on or note-off midi messages in the time domain to match in/out point that just got moved.


## potential feature road map:

1. accepts 1 midi controller as input to N midi tracks
3. can output midi tracks through midi output
    - potentially multiple simultaneously (limited by number of midi out ports? or can multiple midi devices communicate over the same midi interface?)
    - one for each voice
4. long term track storage, i.e. you need to be able to turn it off and then turn it on and have the same midi on disk.


