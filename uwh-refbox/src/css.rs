// Red = #FF0000
// Dark Red = #6C0303
// Orange = FF8500
// Yellow = #FFFF00
// Green = #22FF00
// Lavender = #FF00FF
// Teal = #00FFFB
// Blue = #0000FF
// Light Blue = #4E4EF4
// Dark Blue = 0000DD
// Light Green = #38AA38
// Background Gray = #D3D3D3
// Grayed-Out Gray = #999999
// Dark Gray = #696969
// Black = #000000
// White = #FFFFFF

pub(crate) const STYLE: &str = " 
window {
    background-color: #D3D3D3;
}
grid.white {
    background-color: #FFFFFF;
    border-radius: 5px;
}
grid.black {
    background-color: #000000;
    border-radius: 5px;
}
grid.keypad {
    background-color: #696969;
    border-radius: 5px;
}


button {
    border-style: none;
    outline-style: none;
    box-shadow: none;
    background-color: #FF0000;
    color: #000000;
    background-image: none;
    text-shadow: none;
    -gtk-icon-shadow: none;
    -gtk-icon-effect: none;
    padding: 0px;
}
button:active {
    background-color: #DD0000;
}
button:hover {
    -gtk-icon-shadow: none;
    -gtk-icon-effect: none;
}
button:disabled {
    background-color: #999999;
    color: #696969;
}

button.white-score {
    background-color: #FFFFFF;
    color: #000000;
    border-top-right-radius: 0px;
    border-top-left-radius: 0px;
    font-size: 60px;
    font-weight: bold;
}
button.white-score:active {
    background-color: #696969;
}
button.black-score {
    background-color: #000000;
    color: #FFFFFF;
    border-top-right-radius: 0px;
    border-top-left-radius: 0px;
    font-size: 60px;
    font-weight: bold;
}
button.black-score:active {
    background-color: #696969;
}
button.game-time-green {
    background-color: #696969;
    color: #22FF00;
    border-top-right-radius: 0px;
    border-top-left-radius: 0px;
    font-size: 60px;
    font-weight: bold;
}
button.game-time-green:active {
    background-color: #999999;
}
button.game-time-yellow {
    background-color: #FFFF00;
    color: #22FF00;
    border-top-right-radius: 0px;
    border-top-left-radius: 0px;
    font-size: 60px;
    font-weight: bold;
}
button.game-time-yellow:active {
    background-color: #999999;
}
button.game-time-red {
    background-color: #FF0000;
    color: #22FF00;
    border-top-right-radius: 0px;
    border-top-left-radius: 0px;
    font-size: 60px;
    font-weight: bold;
}
button.game-time-red:active {
    background-color: #999999;
}
button.game-time-gray {
    background-color: #999999;
    color: #000000;
    border-top-right-radius: 0px;
    border-top-left-radius: 0px;
    font-size: 60px;
    font-weight: bold;
}
button.keypad {
    background-color: #00FFFB;
    color: #000000;
    font-size: 50px;
    font-weight: bold;
    border-style: solid;
    border-radius: 5px;
    border-color: #696969;
}
button.keypad:active {
    background-color: #4E4EF4;
    color: #000000;
}
button.white {
    background-color: #FFFFFF;
    color: #000000;
    font-size: 25px;
    font-weight: bold;
}
button.white:active {
    background-color: #999999;
}
button.black {
    background-color: #000000;
    color: #FFFFFF;
    font-size: 25px;
    font-weight: bold;
}
button.black:active {
    background-color: #999999;
}
button.red {
    background-color: #FF0000;
    color: #000000;
    font-size: 25px;
    font-weight: bold;
}
button.red:active {
    background-color: #6C0303;
}
button.little-red {
    background-color: #FF0000;
    color: #000000;
    font-size: 20px;
    font-weight: bold;
}
button.little-red:active {
    background-color: #6C0303;
    border-style: solid;
    border-radius: 5px;
    border-color: #696969;
}
button.orange {
    background-color: #FF8500;
    color: #000000;
    font-size: 25px;
    font-weight: bold;
}
button.orange:active {
    background-color: #DF7500;
}
button.yellow {
    background-color: #FFFF00;
    color: #000000;
    font-size: 25px;
    font-weight: bold;
}
button.yellow:active {
    background-color: #D0D000;
}
button.green {
    background-color: #22FF00;
    color: #000000;
    font-size: 25px;
    font-weight: bold;
}
button.green:active {
    background-color: #1ABB00;
}
button.little-green {
    background-color: #22FF00;
    color: #000000;
    font-size: 20px;
    font-weight: bold;
}
button.little-green:active {
    background-color: #1ABB00;
    border-style: solid;
    border-radius: 5px;
    border-color: #696969;
}
button.blue {
    background-color: #4E4EF4;
    color: #000000;
    font-size: 25px;
    font-weight: bold;
}
button.blue:active {
    background-color: #0000BB;
}
button.little-blue {
    background-color: #4E4EF4;
    color: #000000;
    font-size: 20px;
    font-weight: bold;
}
button.little-blue:active {
    background-color: #0000BB;
    border-style: solid;
    border-radius: 5px;
    border-color: #696969;
}
button.blue-modifier {
    background-color: #4E4EF4;
    color: #000000;
    font-size: 40px;
    font-weight: bold;
}
button.blue-modifier:active {
    background-color: #0000BB;
}
button.gray {
    background-color: #999999;
    color: #000000;
    font-size: 25px;
    font-weight: bold;
}
button.gray:active {
    background-color: #696969;
}
button.gray:disabled {
    background-color: #D3D3D3;
    color: #000000;
}




label.header-white {
    background-color: #FFFFFF;
    color: #000000;
    border-top-right-radius: 5px;
    border-top-left-radius: 5px;
    font-size: 25px;
    font-weight: bold;
}
label.header-black {
    background-color: #000000;
    color: #FFFFFF;
    border-top-right-radius: 5px;
    border-top-left-radius: 5px;
    font-size: 25px;
    font-weight: bold;
}
label.header-gray {
    background-color: #D3D3D3;
    color: #000000;
    border-top-right-radius: 5px;
    border-top-left-radius: 5px;
    font-size: 25px;
    font-weight: bold;
}
label.header-dark-gray {
    background-color: #696969;
    color: #22FF00;
    border-top-right-radius: 5px;
    border-top-left-radius: 5px;
    font-size: 25px;
    font-weight: bold;
}
label.edit-parameter-header {
    background-color: #696969;
    color: #000000;
    font-size: 20px;
    font-weight: bold;
    border-radius: 5px;
}
label.edit-parameter-time {
    background-color: #D3D3D3;
    color: #000000;
    font-size: 25px;
    font-weight: bold;
}
label.player-number-dark-gray {
    background-color: #696969;
    color: #000000;
    font-size: 60px;
    font-weight: bold;
    border-top-right-radius: 5px;
    border-top-left-radius: 5px;
}
label.modified-time-gray {
    background-color: #D3D3D3;
    color: #000000;
    font-size: 60px;
    font-weight: bold;
}
label.modified-score-white {
    background-color: #FFFFFF;
    color: #000000;
    font-size: 60px;
    font-weight: bold;
    border-bottom-right-radius: 5px;
    border-bottom-left-radius: 5px;
}
label.modified-score-black {
    background-color: #000000;
    color: #FFFFFF;
    font-size: 60px;
    font-weight: bold;
    border-bottom-right-radius: 5px;
    border-bottom-left-radius: 5px;
}
";