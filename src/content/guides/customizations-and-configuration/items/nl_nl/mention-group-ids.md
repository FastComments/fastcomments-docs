[related-parameter-start name = 'mentionGroupIds'; type = 'Array<string>'; related-parameter-end]

Een lijst met ids om te gebruiken voor de **@mentions** autocomplete. Handig wanneer je wilt voorkomen dat gebruikers worden getagd als ze geen overlappende groepen hebben.

Wanneer opgegeven, zullen alleen gebruikers uit andere groepen in de autocomplete worden weergegeven nadat het `@`-teken is getypt.

[code-example-start config = {mentionGroupIds: ['yxZAhjzda', 'QT19nXbqB']}; linesToHighlight = [6, 7, 8, 9]; title = 'Limit Groups for Mentions'; code-example-end]