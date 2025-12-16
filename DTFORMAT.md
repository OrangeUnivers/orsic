# DynamicTrack™
## Usage
The DynamicTrack (here shortened with DT) music format will allow for looping and segmenting music, this will allow for customization of how long/short music plays and which parts get repeated.
This also allows for randomizing music and combining **multiple files**.

## Data
DTs will be an Zips with custom extensions.
The Zip will be a combination of the following data:
- Music file/-s
- JSON containing track, segmenting and looping data

## Structure
```
DynamicTrack
 ├─ data.json
 ╰─ tracks/
     ├─ music1.mp3 <─┬─ any music file
     ├─ music2.wav <─╯
     ╰─ ...
```