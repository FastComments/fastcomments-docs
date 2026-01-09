[related-parameter-start name = 'moderationGroupIds'; type = 'Array<string>'; related-parameter-end]

Lista identyfikatorów wygenerowanych na stronie [Grupy moderacji](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups).

Jeśli zostanie określone, komentarze dodane przy użyciu tej konfiguracji będą zawierać ten sam zestaw `moderationGroupIds`.

Jeśli `Moderator` ma zdefiniowaną jedną lub więcej [Grupy moderacji], zobaczy on tylko komentarze na stronie `Moderate Comments` powiązane z jego grupą(ami).

[code-example-start config = {moderationGroupIds: ['mxZAhjzdb', 'FT19nXbqA']}; linesToHighlight = [6, 7, 8, 9]; title = 'Specify Moderation Groups'; code-example-end]