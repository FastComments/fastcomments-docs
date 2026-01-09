[related-parameter-start name = 'moderationGroupIds'; type = 'Array<string>'; related-parameter-end]

Seznam ID-jev, ustvarjenih na strani [Skupine za moderiranje](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups).

Če je določeno, bodo komentarji, oddani z navedeno konfiguracijo, vsebovali isti nabor `moderationGroupIds`.

Če ima `Moderator` definirano eno ali več [Skupine za moderiranje](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups), bo
na strani `Moderate Comments` videl le komentarje, povezane z njegovo skupino ali skupinami.

[code-example-start config = {moderationGroupIds: ['mxZAhjzdb', 'FT19nXbqA']}; linesToHighlight = [6, 7, 8, 9]; title = 'Specify Moderation Groups'; code-example-end]

---