import wave
import argparse
import struct

def embed_code_in_wav(input_wav_path, output_wav_path, code):
    with wave.open(input_wav_path, 'rb') as input_wav:
        params = input_wav.getparams()
        frames = input_wav.readframes(params.nframes)

        with wave.open(output_wav_path, 'wb') as output_wav:
            output_wav.setparams(params)

            # Define a marker to indicate the start of the embedded code
            marker = b'CODE'
            end_marker = b'ENDCODE'

            # Embed the code after the marker in the data chunk
            embedded_frames = bytearray(frames)
            embedded_code = marker + code.encode('utf-8') + end_marker
            embedded_frames[:len(embedded_code)] = embedded_code

            output_wav.writeframes(embedded_frames)

def main():
    parser = argparse.ArgumentParser(description="Embed code in a WAV file")
    parser.add_argument("input_wav_path", help="Path to the input WAV file")
    parser.add_argument("output_wav_path", help="Path to the output WAV file")
    parser.add_argument("code", help="The code to embed")

    args = parser.parse_args()

    embed_code_in_wav(args.input_wav_path, args.output_wav_path, args.code)

if __name__ == "__main__":
    main()
