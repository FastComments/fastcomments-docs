[related-parameter-start name = 'customCSS'; type = 'string'; related-parameter-end]

FastComments はカスタマイズ可能になるよう設計されており、ウィジェットで使用するフォントも例外ではありません。

デフォルトでは、FastComments は幅広いデバイスでできるだけ見栄えが良くなるように `system font stack` を使用します。

独自のフォントを定義するには、[Custom CSS ドキュメント](/guide-customizations-and-configuration.html#custom-css) を参照してください。

そこにはカスタム CSS を定義する方法が記載されており、希望するフォントを設定できるようになります。

#### フォントの定義方法

フォントを上書きするには、`.fast-comments, textarea` セレクタを使用して CSS を定義することをお勧めします。例：

[inline-code-attrs-start title = 'カスタム外部フォントの例'; type = 'css'; inline-code-attrs-end]
[inline-code-start]
@import url('https://fonts.googleapis.com/css2?family=Roboto:wght@300&display=swap');
.fast-comments, textarea {
    font-family: 'Roboto', sans-serif;
}
[inline-code-end]

---