[related-parameter-start name = 'disableImageRedirect'; type = 'boolean'; related-parameter-end]

默认情况下，FastComments 允许用户上传图片。 当用户点击该图片时，FastComments 默认会，
在新标签页中打开该图片以全尺寸显示。将此标志设为 true 会禁用此行为：

[code-example-start config = {disableImageRedirect: true}; linesToHighlight = [6]; title = 'Disable Image Redirect'; code-example-end]

如果您不打算自己捕获图片点击事件（参见 [onImageClicked](#callbacks)），我们建议同时使用一些样式来去除图片看起来可点击的效果。

---