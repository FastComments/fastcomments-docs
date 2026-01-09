[related-parameter-start name = 'moderationGroupIds'; type = 'Array<string>'; related-parameter-end]

En liste over id'er genereret fra [Moderationsgrupper](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups) siden.

Når det er angivet, vil kommentarer, der afgives med den specificerede konfiguration, indeholde det samme sæt `moderationGroupIds`.

Hvis en `Moderator` har en eller flere [Moderationsgrupper](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups) defineret, vil de
kun se kommentarer på siden `Moderate Comments`, som er tilknyttet deres gruppe(r).

[code-example-start config = {moderationGroupIds: ['mxZAhjzdb', 'FT19nXbqA']}; linesToHighlight = [6, 7, 8, 9]; title = 'Specify Moderation Groups'; code-example-end]