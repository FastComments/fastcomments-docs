[related-parameter-start name = 'customCSS'; type = 'string'; related-parameter-end]

FastComments 设计用于可定制，我们小部件使用的字体也不例外。

默认情况下，FastComments 使用 `system font stack`，以便在各种设备上尽可能呈现良好外观。

要定义您自己的字体，请参阅 [自定义 CSS 文档](/guide-customizations-and-configuration.html#custom-css)。

在那里您会找到定义自定义 CSS 的方法，从而允许您设置所需的字体。

#### 如何定义字体

要覆盖字体，我们建议使用 `.fast-comments, textarea` 选择器来定义您的 CSS。例如：

[inline-code-attrs-start title = '自定义外部字体示例'; type = 'css'; inline-code-attrs-end]
[inline-code-start]
@import url('https://fonts.googleapis.com/css2?family=Roboto:wght@300&display=swap');
.fast-comments, textarea {
    font-family: 'Roboto', sans-serif;
}
[inline-code-end]

---