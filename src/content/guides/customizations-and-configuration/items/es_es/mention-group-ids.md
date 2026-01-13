[related-parameter-start name = 'mentionGroupIds'; type = 'Array<string>'; related-parameter-end]

Una lista de ids para usar en el autocompletado de **@mentions**. Útil cuando quieras evitar etiquetar usuarios que no comparten grupos.

Cuando se especifica, solo se incluirán en el autocompletado los usuarios de otros grupos después de escribir el carácter `@`.

[code-example-start config = {mentionGroupIds: ['yxZAhjzda', 'QT19nXbqB']}; linesToHighlight = [6, 7, 8, 9]; title = 'Limit Groups for Mentions'; code-example-end]

---