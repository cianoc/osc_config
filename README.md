# OSC Config

Useful utility application that takes the contents of a RON file and transmits them as OSC.

Written for my own purposes - probably not useful to anyone else (though the code is simple enough). It sends OSC packets based upon the config file when loaded, then watches it for changes. Useful if you have synths in SuperCollider, CSound, etc where you have params driven by OSC and you want to change on the fly.

All params are sent as floats, where the address protocol is the synth name, param name and the value. E.g. if your synth is kick and you have two parameters, the addresses might be:

- /kick/attack
- /kick/release
