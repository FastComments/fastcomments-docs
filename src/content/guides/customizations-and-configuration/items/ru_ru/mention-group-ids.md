---
[related-parameter-start name = 'mentionGroupIds'; type = 'Array<string>'; related-parameter-end]

Список идентификаторов, используемых для автодополнения **@mentions**. Полезно, когда вы хотите предотвратить упоминание пользователей, если у них нет пересекающихся групп.

Когда задано, в автозаполнении после ввода символа `@` будут предлагаться только пользователи из других групп.

[code-example-start config = {mentionGroupIds: ['yxZAhjzda', 'QT19nXbqB']}; linesToHighlight = [6, 7, 8, 9]; title = 'Limit Groups for Mentions'; code-example-end]

---