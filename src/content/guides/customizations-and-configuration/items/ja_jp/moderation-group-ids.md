[related-parameter-start name = 'moderationGroupIds'; type = 'Array<string>'; related-parameter-end]

このリストは[Moderation Groups](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups)ページで生成されたIDの一覧です。

指定すると、その構成を使用して投稿されたコメントには同じセットの `moderationGroupIds` が含まれます。

もし `Moderator` が1つ以上の [Moderation Groups](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups) を定義している場合、  
自分のグループに関連する `Moderate Comments` ページのコメントのみを見ることができます。

[code-example-start config = {moderationGroupIds: ['mxZAhjzdb', 'FT19nXbqA']}; linesToHighlight = [6, 7, 8, 9]; title = 'Specify Moderation Groups'; code-example-end]