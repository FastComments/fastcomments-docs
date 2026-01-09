[related-parameter-start name = 'mentionGroupIds'; type = 'Array<string>'; related-parameter-end]

Списак id-јева за коришћење у аутокомплету за **@mentions**. Корисно када желите да спречите означавање корисника који немају заједничких група.

Када је наведено, у аутокомплету ће након укуцавања `@` карактера бити приказани само корисници из других група.

[code-example-start config = {mentionGroupIds: ['yxZAhjzda', 'QT19nXbqB']}; linesToHighlight = [6, 7, 8, 9]; title = 'Limit Groups for Mentions'; code-example-end]