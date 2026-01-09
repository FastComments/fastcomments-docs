---
[related-parameter-start name = 'moderationGroupIds'; type = 'Array<string>'; related-parameter-end]

Списък с идентификатори, генерирани от страницата [Групи за модерация](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups).

Когато е посочено, коментарите, оставени с използване на посочената конфигурация, ще съдържат същия набор от `moderationGroupIds`.

Ако един `Moderator` има дефинирани една или повече [Групи за модерация](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups), те ще
виждат само коментарите в страницата `Moderate Comments`, свързани с техните групи.

[code-example-start config = {moderationGroupIds: ['mxZAhjzdb', 'FT19nXbqA']}; linesToHighlight = [6, 7, 8, 9]; title = 'Specify Moderation Groups'; code-example-end]

---