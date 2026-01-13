[related-parameter-start name = 'mentionGroupIds'; type = 'Array<string>'; related-parameter-end]

Une liste d'ids à utiliser pour l'autocomplétion des **@mentions**. Utile lorsque vous souhaitez empêcher de taguer des utilisateurs qui n'ont pas de groupes communs.

Si spécifié, seuls les utilisateurs appartenant à d'autres groupes seront proposés dans l'autocomplétion après avoir tapé le caractère `@`.

[code-example-start config = {mentionGroupIds: ['yxZAhjzda', 'QT19nXbqB']}; linesToHighlight = [6, 7, 8, 9]; title = 'Limit Groups for Mentions'; code-example-end]

---