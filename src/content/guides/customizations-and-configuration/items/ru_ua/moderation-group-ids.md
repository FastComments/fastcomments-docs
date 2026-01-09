[related-parameter-start name = 'moderationGroupIds'; type = 'Array<string>'; related-parameter-end]

Список идентификаторов, сгенерированных на странице [Группы модерации](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups).

Когда указано, комментарии, оставленные с использованием указанной конфигурации, будут содержать тот же набор `moderationGroupIds`.

Если у `Moderator` определена одна или несколько [Группы модерации](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups), они будут
видеть только комментарии на странице `Moderate Comments`, связанные с их группой(ами).

[code-example-start config = {moderationGroupIds: ['mxZAhjzdb', 'FT19nXbqA']}; linesToHighlight = [6, 7, 8, 9]; title = 'Specify Moderation Groups'; code-example-end]