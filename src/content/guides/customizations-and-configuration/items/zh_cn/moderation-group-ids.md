[related-parameter-start name = 'moderationGroupIds'; type = 'Array<string>'; related-parameter-end]

一个由 [审核组](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups) 页面生成的 id 列表。

当指定后，使用该配置留下的评论将包含相同的一组 `moderationGroupIds`。

如果某个 `Moderator` 定义了一个或多个 [审核组](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups)，他们将
只会在与其分组相关联的 `Moderate Comments` 页面中看到评论。

[code-example-start config = {moderationGroupIds: ['mxZAhjzdb', 'FT19nXbqA']}; linesToHighlight = [6, 7, 8, 9]; title = 'Specify Moderation Groups'; code-example-end]