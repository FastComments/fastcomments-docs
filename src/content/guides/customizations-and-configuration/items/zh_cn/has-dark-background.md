[related-parameter-start name = 'hasDarkBackground'; type = 'boolean'; related-parameter-end]

默认情况下，FastComments 评论组件会在大多数网站上自动检测深色模式。

检测到深色模式时，FastComments 会将黑色文字/白色背景切换为黑色背景/白色文字。图像也会随之改变。

在页面加载时，组件会尝试判断评论组件背后页面背景的明暗程度。这意味着
页面可能是白色背景，但如果你将评论组件放在一个黑色背景的容器内，深色模式仍然应该
会自动启用以确保评论可读。

不过，依赖于“亮度”判断的检测机制可能不会在你期望的时候启用深色模式。要强制启用它，请将
*hasDarkBackground* 标志设置为 true，如下所示：

[code-example-start config = {hasDarkBackground: true}; linesToHighlight = [6]; title = 'Force Dark Background Mode'; additionalDemoCode = '<style>body { background: black; }</style>'; code-example-end]

---