# Third-party model notice

`u2netp.onnx` in this directory is **not** created by this project. It is
embedded as-is (via `include_bytes!` in `../src/tools/ort_segmentation_tool.rs`)
to perform background removal.

## Model: U²-Net (u2netp variant)

- Authors: Xuebin Qin, Zichen Zhang, Chenyang Huang, Masood Dehghan, Osmar
  Zaiane, Martin Jagersand.
- Paper: *"U²-Net: Going Deeper with Nested U-Structure for Salient Object
  Detection"*, Pattern Recognition, vol. 106, p. 107404, 2020.
- Source: https://github.com/xuebinqin/U-2-Net
- License: Apache License 2.0 — full text in `./LICENSE-u2net` (copied
  verbatim from the source repository above).

```bibtex
@InProceedings{Qin_2020_PR,
title = {U2-Net: Going Deeper with Nested U-Structure for Salient Object Detection},
author = {Qin, Xuebin and Zhang, Zichen and Huang, Chenyang and Dehghan, Masood and Zaiane, Osmar and Jagersand, Martin},
journal = {Pattern Recognition},
volume = {106},
pages = {107404},
year = {2020}
}
```

## Model file source: rembg

The specific `.onnx` build embedded here was not converted by this project
either — it's taken directly from
[danielgatis/rembg](https://github.com/danielgatis/rembg)'s GitHub Releases
(`u2netp.onnx`, MD5 `8e83ca70e441ab06c318d82300c84806`), which packages
several pretrained background-removal models as ready-to-use ONNX files.
rembg is licensed under the MIT License
(https://github.com/danielgatis/rembg/blob/main/LICENSE.txt).

See the root `README.md`'s "Credits" section for the same attribution in
context.
