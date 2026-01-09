[related-parameter-start name = 'noNewRootComments'; type = 'boolean'; related-parameter-end]

将 `noNewRootComments` 设置为 `true` 会导致小部件隐藏顶层回复区域，但仍允许用户回复
子评论。你可以例如在页面加载时有条件地设置此项，只允许某些用户留下顶级评论。

[code-example-start config = {noNewRootComments: true}; linesToHighlight = [6]; title = 'Prevent New Root Comments'; code-example-end]

---