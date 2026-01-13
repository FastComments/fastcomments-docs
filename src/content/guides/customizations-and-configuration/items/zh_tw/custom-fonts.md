[related-parameter-start name = 'customCSS'; type = 'string'; related-parameter-end]

FastComments 設計上可供客製化，我們小工具使用的字型也不例外。

預設情況下，FastComments 使用 `system font stack`，以在各種裝置上呈現最佳外觀。

若要定義您自己的字型，請參閱 [自訂 CSS 文件](/guide-customizations-and-configuration.html#custom-css)。

在該處您會找到定義自訂 CSS 的方法，從而讓您能指定所需的字型。

#### 如何定義字型

要覆寫字型，我們建議使用 `.fast-comments, textarea` 選擇器來定義您的 CSS。範例如下：

[inline-code-attrs-start title = '自訂外部字型範例'; type = 'css'; inline-code-attrs-end]
[inline-code-start]
@import url('https://fonts.googleapis.com/css2?family=Roboto:wght@300&display=swap');
.fast-comments, textarea {
    font-family: 'Roboto', sans-serif;
}
[inline-code-end]

---