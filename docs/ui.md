# modes

the UI has two "modes":

1. midi port select (input and output.)
2. tape, which includes:
    - input mode, wherein all midi messages are output to the output port. There's also a sub-mode,
        - recording, wherein all midi messages are output, but also saved to the track data structure.
    - playback mode

# message types:

## port selection:

- PortSelected (i.e. `->` key is hit on a highlighted port.) (triggers movement to tape mode interface.)
- PortScrollDown (i.e. an arrow key or the scrollwheel is moved, and the list of ports should be moved down.)
- PortScrollUp (i.e. an arrow key or the scrollwheel is moved, and the list of ports should be moved up.)

## tape:

### config changes
- OpenInputSelect (`i` is hit, open the input selection menu)
- OpenOutputSelect (`o` is hit, open the output selection menu)
- BpmSelect (focus the BPM window, accept typed numbers or scrolling/arrow keys, then hit `CR` to exit.)
    - note that this is actually even a third mode, wherein all other keybindings stop working and you need to hit `CR` to re-enter tape mode again.

### track options
- TrackSelect (hit a number key, select the relevant track. Can also hit `t` and then type for track numbers greater than 9)
- TrackScroll (scroll the mousewheel or hit `,` or `.` (the uncapitalized versions of `<` and `>`), move the current time window across the selected track)
- TrackZoomIn (`=` (aka uncapitalized `+`) is hit, zoom the track view in (midi messages get bigger).)
- TrackZoomOut (`-` is hit, zoom the track view out (midi messages get smaller.)
- SetInPoint (`[` is hit, set the `in` point on the track.)
- SetOutPoint (`]` is hit, set the `out` point on the track.)
- CutSelection
- CopySelection
- PasteSelection

### mode changes

- TogglePlayback (`space` is hit, toggle playback (start playing if paused or stopped, stop playing if currently playing.)
    - this needs to happen properly even if we are currently recording (i.e. recording is a subset of this state.)
- BeginRecordingAtCurrentPoint (`r` is hit, begin recording at the time this message arrives)
- BeginRecordingUponMessage (`R` is hit, begin recording when a new NoteOn message arrives.)
- StopRecording (`s` is hit, pause and exit recording mode.)



I think this could be all? I don't love their verbosity, but I also want them to be extremely clear what they are doing, because that should help me when reasoning about the GUI itself.

# how does this integrate with the rest of the application?

well, the elm architecture puts the UI as the primary thing that dispatches messages to the state.

So, we need:

a state model, which in our case is going to be an object like this:

```
struct MultiTrack {
    bpm: u8,
    in: u32,
    out: u32,
    cursor: u32, //  the center of the window
    zoom: u32,   //  the width of our window, centered on `cursor`.

    tracks: List<Hashmap<u32,Message>>
    /* Here you can look up the "message that should be output" by looking up the current timestampe in each Hashmap.
     * Each message is stored under the timestamp that it should be output, so each `tick` that the program runs, we are
     * looking for the current timestamp in each HashMap (each HashMap is a track.)
     */
    time: u32, //  the "local" time (this 
}

Struct TrackMsg {
    duration: u32,
    pitch: u16,
    velocity: u8,
}
