[related-parameter-start name = 'moderationGroupIds'; type = 'Array<string>'; related-parameter-end]

Une liste d'identifiants générés à partir de la page [Groupes de modération](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups).

Lorsqu'elles sont spécifiées, les commentaires laissés en utilisant la configuration spécifiée contiendront le même ensemble de `moderationGroupIds`.

Si un `Moderator` a défini un ou plusieurs [Groupes de modération](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups), il ou elle ne verra que les commentaires de la page `Moderate Comments` associés à son ou ses groupe(s).

[code-example-start config = {moderationGroupIds: ['mxZAhjzdb', 'FT19nXbqA']}; linesToHighlight = [6, 7, 8, 9]; title = 'Specify Moderation Groups'; code-example-end]