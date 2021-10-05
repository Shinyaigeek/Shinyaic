# css border property

https://drafts.csswg.org/css-backgrounds-3/#the-border-shorthands

> The border can either be a predefined style (solid line, double line, dotted line, pseudo-3D border, etc.) or it can be an image. In the former case, various properties define the style (border-style), color (border-color), and thickness (border-width) of the border.

border には規定のスタイル(solidとか)も使うことができるし, 画像を利用することもできる.

- border-style
- border-color
- border-width 

をcssで定義する.

## border color

初期値は `currentColor`, blackではない.

ruby base container, ruby annotation container 以外に適用できる.

`border-top-color` などで, 局所的な border の色も変えられる.
ていうか `border-color` は `border-top-color`, `border-bottom-color` など4種のショートハンド

## border style

border の見え方を扱う.

ruby base container, ruby annotation container 以外に適用できる.
初期値はnone(特になし)

border color と同様に四方できる.

### とりうる値

- none
- solid
- dotted
- groove
- ridge
- dashed
- inset
- outset
- double

## border width

border の太さを表す, line-width 値をとりうる.

ruby base container, ruby annotation container 以外に適用できる.

0 ~ Infinity, thin, medium, thick の値をとりうる

thin, medium. thick の具体的な値は定義されていない

