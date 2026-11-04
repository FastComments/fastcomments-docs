[related-parameter-start name = 'maxReplyDepth'; type = 'number'; related-parameter-end]

預設情況下，FastComments 允許無限層級的回覆嵌套，建立一個使用者可以無限回覆回覆的討論串結構。

maxReplyDepth 選項允許您限制回覆討論串的深度。當達到最大深度時，使用者在該層級的留言上將不再看到回覆按鈕。

[code-example-start config = {maxReplyDepth: 2}; linesToHighlight = [6]; title = '將回覆深度限制為 2 層'; code-example-end]

當 maxReplyDepth 設定為 2 時：
- 使用者可以在頂層留言（深度 0）
- 使用者可以回覆頂層留言（深度 1）
- 使用者可以回覆那些回覆（深度 2）
- 超過深度 2 後不允許再回覆

設定為 1 則僅允許回覆頂層留言，建立更扁平的討論結構。

將 maxReplyDepth 設定為 0 將停用所有回覆，僅允許頂層留言。若未指定，回覆可以無限嵌套。