import wave
import argparse

def embed_code_in_wav(input_wav_path, output_wav_path, encrypted_code):
    with wave.open(input_wav_path, 'rb') as input_wav:
        params = input_wav.getparams()
        frames = input_wav.readframes(params.nframes)
        
        with wave.open(output_wav_path, 'wb') as output_wav:
            output_wav.setparams(params)
            
            output_wav.setsampwidth(params.sampwidth)
            output_wav.setnchannels(params.nchannels)
            output_wav.setframerate(params.framerate)
            output_wav.setnframes(params.nframes)
            output_wav.setcomptype(params.comptype, encrypted_code)
            
            output_wav.writeframes(frames)

def main():
    parser = argparse.ArgumentParser(description="Embed encrypted code in a WAV file")
    parser.add_argument("input_wav_path", help="Path to the input WAV file")
    parser.add_argument("output_wav_path", help="Path to the output WAV file")
    parser.add_argument("encrypted_code", help="The encrypted code to embed")

    args = parser.parse_args()
    
    embed_code_in_wav(args.input_wav_path, args.output_wav_path, args.encrypted_code)

if __name__ == "__main__":
    main()