[related-parameter-start name = 'moderationGroupIds'; type = 'Array<string>'; related-parameter-end]

Una lista de identificadores generados desde la página [Grupos de Moderación](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups).

Cuando se especifique, los comentarios publicados usando la configuración especificada contendrán el mismo conjunto de `moderationGroupIds`.

Si un `Moderator` tiene uno o más [Grupos de Moderación](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups) definidos, solo
verán los comentarios en la página `Moderate Comments` asociados con su(s) grupo(s).

[code-example-start config = {moderationGroupIds: ['mxZAhjzdb', 'FT19nXbqA']}; linesToHighlight = [6, 7, 8, 9]; title = 'Specify Moderation Groups'; code-example-end]

---