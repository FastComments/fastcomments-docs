[related-parameter-start name = 'moderationGroupIds'; type = 'Array<string>'; related-parameter-end]

Popis ID-jeva generiranih sa stranice [Grupe za moderiranje](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups).

Kada je navedeno, komentari ostavljeni koristeći navedenu konfiguraciju sadržavat će isti skup `moderationGroupIds`.

Ako `Moderator` ima jednu ili više [Grupe za moderiranje](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups) definirane, oni će
vidjeti samo komentare na stranici `Moderate Comments` povezane s njihovim grupama.

[code-example-start config = {moderationGroupIds: ['mxZAhjzdb', 'FT19nXbqA']}; linesToHighlight = [6, 7, 8, 9]; title = 'Specify Moderation Groups'; code-example-end]

---