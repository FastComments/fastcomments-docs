---
[related-parameter-start name = 'moderationGroupIds'; type = 'Array<string>'; related-parameter-end]

Een lijst met id's die zijn gegenereerd vanaf de [Moderatiegroepen](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups) pagina.

Wanneer opgegeven, bevatten opmerkingen die met de opgegeven configuratie zijn geplaatst dezelfde set van `moderationGroupIds`.

Als een `Moderator` één of meer [Moderatiegroepen](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups) heeft gedefinieerd, zullen zij
alleen opmerkingen zien op de pagina `Moderate Comments` die gekoppeld zijn aan hun groep(en).

[code-example-start config = {moderationGroupIds: ['mxZAhjzdb', 'FT19nXbqA']}; linesToHighlight = [6, 7, 8, 9]; title = 'Specify Moderation Groups'; code-example-end]

---