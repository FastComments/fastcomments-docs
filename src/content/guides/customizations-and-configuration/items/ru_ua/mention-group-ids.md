[related-parameter-start name = 'mentionGroupIds'; type = 'Array<string>'; related-parameter-end]

Список идентификаторов, используемых для автозаполнения **@упоминаний**. Полезно, когда вы хотите предотвратить отметку пользователей, если у них нет пересекающихся групп.

Если указано, в автозаполнении после ввода символа `@` будут предлагаться только пользователи из других групп.

[code-example-start config = {mentionGroupIds: ['yxZAhjzda', 'QT19nXbqB']}; linesToHighlight = [6, 7, 8, 9]; title = 'Limit Groups for Mentions'; code-example-end]

---