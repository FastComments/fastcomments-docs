[related-parameter-start name = 'maxReplyDepth'; type = 'number'; related-parameter-end]

預設情況下，FastComments 允許無限制的回覆層級，會建立一個討論串結構，使用者可以無限地對回覆再進行回覆。

The maxReplyDepth option allows you to limit how deep reply threads can go. When the maximum depth is reached, users will no longer see a reply button on comments at that level.

[code-example-start config = {maxReplyDepth: 2}; linesToHighlight = [6]; title = 'Limiting Reply Depth to 2 Levels'; code-example-end]

當 maxReplyDepth 設為 2：
- 使用者可以在頂層發表評論（深度 0）
- 使用者可以回覆頂層評論（深度 1）
- 使用者可以回覆那些回覆（深度 2）
- 深度超過 2 不允許再有進一步回覆

Setting to 1 would only allow replies to top-level comments, creating a flatter discussion structure.

Setting maxReplyDepth to 0 would disable all replies, allowing only top-level comments. If not specified, replies can be nested without limit.