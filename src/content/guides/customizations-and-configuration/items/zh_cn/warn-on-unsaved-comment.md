[related-parameter-start name = 'warnOnUnsavedComment'; type = 'boolean'; related-parameter-end]

默认情况下，如果用户输入评论后在提交前刷新页面、关闭标签页或离开页面，草稿会悄悄丢失。

将 **warnOnUnsavedComment** 设置为 true 时，只要任何评论框或正在编辑的内容仍包含文本，浏览器在离开页面前会要求用户确认。评论提交后文本会被清除，因此不会显示提示。

[code-example-start config = {warnOnUnsavedComment: true}; linesToHighlight = [6]; title = '警告未保存的评论'; code-example-end]

提示使用浏览器自身的对话框。现代浏览器显示自己的文字并忽略自定义文本，因此无法自定义该消息。

此选项按需加载一个小扩展，对于未启用该功能的站点，不会向小部件添加任何内容。