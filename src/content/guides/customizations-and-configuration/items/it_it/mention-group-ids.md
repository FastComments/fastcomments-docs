[related-parameter-start name = 'mentionGroupIds'; type = 'Array<string>'; related-parameter-end]

Una lista di ID da usare per l'autocompletamento delle **@mentions**. Utile quando vuoi evitare di taggare utenti che non hanno gruppi in comune.

Quando specificato, solo gli utenti appartenenti ad altri gruppi verranno mostrati nell'autocompletamento dopo aver digitato il carattere `@`.

[code-example-start config = {mentionGroupIds: ['yxZAhjzda', 'QT19nXbqB']}; linesToHighlight = [6, 7, 8, 9]; title = 'Limit Groups for Mentions'; code-example-end]