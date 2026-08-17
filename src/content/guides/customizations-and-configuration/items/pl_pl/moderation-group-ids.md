[related-parameter-start name = 'moderationGroupIds'; type = 'Array<string>'; related-parameter-end]

Lista identyfikatorów wygenerowanych z strony [Moderation Groups](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups).

Gdy zostanie określone, komentarze pozostawione przy użyciu określonej konfiguracji będą zawierały ten sam zestaw `moderationGroupIds`.

Jeśli `Moderator` ma jedną lub więcej zdefiniowanych [Moderation Groups](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups) defined, będą
widzieć tylko komentarze na stronie `Moderate Comments` powiązane ze swoją grupą (grupami).

[code-example-start config = {moderationGroupIds: ['mxZAhjzdb', 'FT19nXbqA']}; linesToHighlight = [6, 7, 8, 9]; title = 'Określ grupy moderacji'; code-example-end]

---