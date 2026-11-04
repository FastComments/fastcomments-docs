[related-parameter-start name = 'maxReplyDepth'; type = 'number'; related-parameter-end]

默认情况下，FastComments 允许无限层级的回复嵌套，形成用户可以无限回复回复的线程结构。

maxReplyDepth 选项允许您限制回复线程的最大深度。当达到最大深度时，用户将不再能在该层级的评论上看到回复按钮。

[code-example-start config = {maxReplyDepth: 2}; linesToHighlight = [6]; title = '将回复深度限制为 2 层'; code-example-end]

将 maxReplyDepth 设置为 2 时：
- 用户可以在顶层发表评论（深度 0）
- 用户可以回复顶层评论（深度 1）
- 用户可以回复这些回复（深度 2）
- 超过深度 2 后不允许再回复

设置为 1 则只允许回复顶层评论，形成更扁平的讨论结构。

将 maxReplyDepth 设置为 0 将禁用所有回复，只允许顶层评论。如果未指定，回复可以无限嵌套。