# Rudeloder

Test of steganographic Audio File Code-Injection,
You can inject any Rust code into the WAV file and run it by loading the WAV file with Rudeloader.exe

## Usage

1. You can inject any Rust code into a WAV file using injector.py  
`$ python ./injector.py ./tr1.wav ./tr1_ev.wav "fn main() { println!("Hello World!"); }" ` 

2. Any code is fired by executing the WAV file with the code injected as the first argument to rudeloader.exe  
`$ .\target\debug\rudeloder.exe .\tr1_ev.wav`  
`Code executed successfully: Hello World!`


You may use this code for research purposes only.
