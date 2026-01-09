[related-parameter-start name = 'moderationGroupIds'; type = 'Array<string>'; related-parameter-end]

Una lista di ID generati dalla pagina [Gruppi di moderazione](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups).

Quando specificato, i commenti lasciati usando la configurazione specificata conterranno lo stesso insieme di `moderationGroupIds`.

Se un `Moderator` ha uno o più [Gruppi di moderazione](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups) definiti, essi
vedranno solo i commenti nella pagina `Moderate Comments` associati ai loro gruppi.

[code-example-start config = {moderationGroupIds: ['mxZAhjzdb', 'FT19nXbqA']}; linesToHighlight = [6, 7, 8, 9]; title = 'Specify Moderation Groups'; code-example-end]