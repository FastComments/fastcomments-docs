[related-parameter-start name = 'moderationGroupIds'; type = 'Array<string>'; related-parameter-end]

Lista id-eva generisanih sa stranice [Grupe za moderaciju](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups).

Kada je navedeno, komentari ostavljeni koristeći navedenu konfiguraciju imaće isti skup `moderationGroupIds`.

Ako `Moderator` ima jednu ili više definisanih [Grupe za moderaciju](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups), oni će
videti samo komentare na stranici `Moderate Comments` koji su povezani sa njihovom grupom/grupama.

[code-example-start config = {moderationGroupIds: ['mxZAhjzdb', 'FT19nXbqA']}; linesToHighlight = [6, 7, 8, 9]; title = 'Specify Moderation Groups'; code-example-end]