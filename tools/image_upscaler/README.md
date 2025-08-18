# Image Upscaler

A simple standalone script to enlarge images using [Pillow](https://python-pillow.org/).

## Setup

Install dependencies:

```bash
pip install -r requirements.txt
```

## Usage

```bash
python upscale.py input.jpg output.png --scale 2
```

The example above doubles the size of `input.jpg` and writes the result to `output.png`. The `--scale`
argument controls the upscale factor.
