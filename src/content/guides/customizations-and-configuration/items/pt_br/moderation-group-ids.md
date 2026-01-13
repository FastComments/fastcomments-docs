[related-parameter-start name = 'moderationGroupIds'; type = 'Array<string>'; related-parameter-end]

Uma lista de ids gerados a partir da página [Grupos de Moderação](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups).

Quando especificado, os comentários deixados usando a configuração especificada conterão o mesmo conjunto de `moderationGroupIds`.

Se um `Moderator` tiver um ou mais [Grupos de Moderação](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups) definidos, ele(a) verá apenas os comentários na página `Moderate Comments` associados ao(s) seu(s) grupo(s).

[code-example-start config = {moderationGroupIds: ['mxZAhjzdb', 'FT19nXbqA']}; linesToHighlight = [6, 7, 8, 9]; title = 'Specify Moderation Groups'; code-example-end]

---