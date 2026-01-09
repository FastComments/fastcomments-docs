[related-parameter-start name = 'moderationGroupIds'; type = 'Array<string>'; related-parameter-end]

Lista ID-ova generisanih sa stranice [Grupe za moderaciju](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups).

Kada su navedeni, komentari ostavljeni koristeći navedenu konfiguraciju će sadržati isti skup `moderationGroupIds`.

Ako `Moderator` ima definisanu jednu ili više [Grupe za moderaciju](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups), oni će
vidjeti samo komentare na stranici `Moderate Comments` koji su povezani sa njihovom grupom ili grupama.

[code-example-start config = {moderationGroupIds: ['mxZAhjzdb', 'FT19nXbqA']}; linesToHighlight = [6, 7, 8, 9]; title = 'Specify Moderation Groups'; code-example-end]