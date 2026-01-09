[related-parameter-start name = 'moderationGroupIds'; type = 'Array<string>'; related-parameter-end]

Списак ИД-ова генерисаних са странице [Групе за модерацију](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups).

Када је наведено, коментари остављени користећи ту конфигурацију имаће исти скуп `moderationGroupIds`.

Ако `Moderator` има дефинисану једну или више [Групе за модерацију](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups), они ће видети само коментаре на страници `Moderate Comments` који су повезани са њиховом групом(ама).

[code-example-start config = {moderationGroupIds: ['mxZAhjzdb', 'FT19nXbqA']}; linesToHighlight = [6, 7, 8, 9]; title = 'Specify Moderation Groups'; code-example-end]

---