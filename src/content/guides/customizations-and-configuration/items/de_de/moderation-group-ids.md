[related-parameter-start name = 'moderationGroupIds'; type = 'Array<string>'; related-parameter-end]

Eine Liste von IDs, die von der Seite [Moderationsgruppen](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups) generiert werden.

Wenn angegeben, enthalten Kommentare, die mit der angegebenen Konfiguration hinterlassen werden, denselben Satz von `moderationGroupIds`.

Wenn ein `Moderator` eine oder mehrere [Moderationsgruppen](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups) definiert hat, werden
sie nur Kommentare auf der Seite `Moderate Comments` sehen, die mit ihrer Gruppe(n) verknüpft sind.

[code-example-start config = {moderationGroupIds: ['mxZAhjzdb', 'FT19nXbqA']}; linesToHighlight = [6, 7, 8, 9]; title = 'Specify Moderation Groups'; code-example-end]

---