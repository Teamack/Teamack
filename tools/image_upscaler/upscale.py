#!/usr/bin/env python3
"""Simple command-line image upscaler using Pillow."""

import argparse
from pathlib import Path
from PIL import Image

def upscale_image(input_path: Path, output_path: Path, scale: float) -> None:
    """Upscale an image by the provided scale factor and save it."""
    img = Image.open(input_path)
    new_size = (int(img.width * scale), int(img.height * scale))
    upscaled = img.resize(new_size, Image.LANCZOS)
    upscaled.save(output_path)


def main() -> None:
    parser = argparse.ArgumentParser(description="Upscale an image by a given factor.")
    parser.add_argument("input", type=Path, help="Path to the input image file")
    parser.add_argument("output", type=Path, help="Path to write the upscaled image")
    parser.add_argument("--scale", type=float, default=2.0, help="Scale factor (default: 2.0)")
    args = parser.parse_args()

    if args.scale <= 0:
        raise ValueError("Scale factor must be positive")

    upscale_image(args.input, args.output, args.scale)


if __name__ == "__main__":
    main()
