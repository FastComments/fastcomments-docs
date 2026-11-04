コメントボックスの周囲の枠線は、テキストエリア自体の枠線と、ウィジェットがその周囲に描画する数本の細い線で構成されています。色や角の丸みを変更するには、`textarea` を直接スタイルするのではなく、以下の CSS 変数を設定してください。これらの変数は枠線のすべての部分を一度に再スタイルするため、角と色は常に一致します。

| 変数 | 変更される内容 | デフォルト |
|---|---|---|
| `--fc-input-border-color` | 枠線の色 | `#bfbfbf` |
| `--fc-input-border-color-focus` | ユーザーが入力中の枠線の色 | `#555` |
| `--fc-input-border-radius` | 角の丸み | `11px` |
| `--fc-input-border-start-start-radius` | 左上の角(右から左に読む言語では右上)の角ばった部分の丸み | `0` |

CSS を [Widget Customization ページ](https://fastcomments.com/auth/my-account/customize-widget) の **Custom CSS** ボックスに追加するか、`customCSS` オプションで渡してください。変更したい変数だけを設定すれば十分です。

また、Custom CSS ボックスのすぐ下にある **Comment box border** ヘルパーを使用することもできます。これはこの CSS を自動的に記述してくれます。

## 枠線の色を変更する

[inline-code-attrs-start title = '枠線の色'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color: #d1d5db;
}
[inline-code-end]

## 入力中の枠線の色を変更する

[inline-code-attrs-start title = '入力中の枠線の色'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color-focus: #2563eb;
}
[inline-code-end]

## 4つの角すべてを丸くする

デフォルトでは左上の角は角ばっています。4つの角すべてを同じように丸くするには、両方の radius 変数を設定してください:

[inline-code-attrs-start title = '4つの角すべてを丸くする'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-radius: 12px;
    --fc-input-border-start-start-radius: 12px;
}
[inline-code-end]

## 角を角ばったままにする

[inline-code-attrs-start title = '角ばった角'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-radius: 0;
}
[inline-code-end]

## ブランドに合わせる

[inline-code-attrs-start title = 'ブランドの色と角'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color: #c7d2fe;
    --fc-input-border-color-focus: #4f46e5;
    --fc-input-border-radius: 8px;
    --fc-input-border-start-start-radius: 8px;
}
[inline-code-end]

## ダークモードで異なる色を使う

ウィジェットがダークモードのときは `dark` クラスが付くため、ダークモード用に異なる値を設定できます:

[inline-code-attrs-start title = 'ダークモードの枠線の色'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color: #d1d5db;
}
.dark {
    --fc-input-border-color: #444;
    --fc-input-border-color-focus: #aaa;
}
[inline-code-end]

## テキストエリアを直接スタイルしないのはなぜ?

ウィジェットは、テキストエリアの周囲のコメントボックスの枠線の一部を自ら描画しています。`textarea` にのみ `border-color` や `border-radius` を設定すると、それらの線はデフォルトのスタイルのままになり、枠線が不揃いに見えます。例えば、丸い角の中に角ばった線が走るような状態です。上記の変数は両方を一度に変更します。