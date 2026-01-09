[related-parameter-start name = 'moderationGroupIds'; type = 'Array<string>'; related-parameter-end]

Список ідентифікаторів, згенерованих на сторінці [Групи модерації](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups).

Коли вказано, коментарі, залишені з використанням цієї конфігурації, міститимуть той самий набір `moderationGroupIds`.

Якщо у `Moderator` визначено одну або кілька [Групи модерації](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups), вони
бачитимуть лише коментарі на сторінці `Moderate Comments`, пов'язані з їхньою групою(групами).

[code-example-start config = {moderationGroupIds: ['mxZAhjzdb', 'FT19nXbqA']}; linesToHighlight = [6, 7, 8, 9]; title = 'Specify Moderation Groups'; code-example-end]