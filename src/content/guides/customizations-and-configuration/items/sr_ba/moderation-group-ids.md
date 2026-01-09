[related-parameter-start name = 'moderationGroupIds'; type = 'Array<string>'; related-parameter-end]

Листа ид-ова генерисаних са странице [Групе за модерацију](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups).

Када је назначено, коментари остављени користећи наведену конфигурацију ће садржати исти скуп `moderationGroupIds`.

Ако `Moderator` има једну или више дефинисаних [Група за модерацију](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups), видеће само коментаре на страници `Moderate Comments` који су повезани са његовом/њеном групом(ама).

[code-example-start config = {moderationGroupIds: ['mxZAhjzdb', 'FT19nXbqA']}; linesToHighlight = [6, 7, 8, 9]; title = 'Specify Moderation Groups'; code-example-end]

---