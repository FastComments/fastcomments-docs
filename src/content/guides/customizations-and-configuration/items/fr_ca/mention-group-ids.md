[related-parameter-start name = 'mentionGroupIds'; type = 'Array<string>'; related-parameter-end]

Une liste d'identifiants à utiliser pour l'autocomplétion des **@mentions**. Utile lorsque vous voulez empêcher de taguer des utilisateurs qui n'ont pas de groupes en commun.

Lorsqu'elle est spécifiée, seuls les utilisateurs appartenant à d'autres groupes seront proposés dans l'autocomplétion après avoir tapé le caractère `@`.

[code-example-start config = {mentionGroupIds: ['yxZAhjzda', 'QT19nXbqB']}; linesToHighlight = [6, 7, 8, 9]; title = 'Limit Groups for Mentions'; code-example-end]

---