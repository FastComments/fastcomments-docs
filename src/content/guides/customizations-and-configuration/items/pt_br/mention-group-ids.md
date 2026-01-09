[related-parameter-start name = 'mentionGroupIds'; type = 'Array<string>'; related-parameter-end]

Uma lista de ids para usar no autocompletar de **@mentions**. Útil quando você quer evitar marcar usuários que não possuem grupos em comum.

Quando especificado, apenas usuários de outros grupos serão exibidos no autocompletar após digitar o caractere `@`.

[code-example-start config = {mentionGroupIds: ['yxZAhjzda', 'QT19nXbqB']}; linesToHighlight = [6, 7, 8, 9]; title = 'Limit Groups for Mentions'; code-example-end]