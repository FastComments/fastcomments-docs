[related-parameter-start name = 'maxReplyDepth'; type = 'number'; related-parameter-end]

默认情况下，FastComments 允许回复无限嵌套，形成一个线程结构，用户可以无限地对回复进行回复。

maxReplyDepth 选项允许您限制回复线程的最大深度。当达到最大深度时，用户将不会在该级别的评论上看到回复按钮。

[code-example-start config = {maxReplyDepth: 2}; linesToHighlight = [6]; title = 'Limiting Reply Depth to 2 Levels'; code-example-end]

With maxReplyDepth set to 2:
- 用户可以在顶层发表评论 (depth 0)
- 用户可以回复顶级评论 (depth 1)
- 用户可以回复这些回复 (depth 2)
- 超出 depth 2 不再允许进一步回复

Setting to 1 would only allow replies to top-level comments, creating a flatter discussion structure.

Setting maxReplyDepth to 0 would disable all replies, allowing only top-level comments. If not specified, replies can be nested without limit.