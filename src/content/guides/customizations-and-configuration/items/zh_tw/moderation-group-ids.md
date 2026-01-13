---
[related-parameter-start name = 'moderationGroupIds'; type = 'Array<string>'; related-parameter-end]

從 [審核群組](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups) 頁面產生的 id 列表。

若有指定，使用該設定留下的評論將包含相同的 `moderationGroupIds` 集合。

如果一位 `Moderator` 定義了一個或多個 [審核群組](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups)，他們將會
只在 `Moderate Comments` 頁面中看到與其群組相關聯的評論。

[code-example-start config = {moderationGroupIds: ['mxZAhjzdb', 'FT19nXbqA']}; linesToHighlight = [6, 7, 8, 9]; title = 'Specify Moderation Groups'; code-example-end]
---