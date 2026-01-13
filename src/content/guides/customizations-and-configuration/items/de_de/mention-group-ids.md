[related-parameter-start name = 'mentionGroupIds'; type = 'Array<string>'; related-parameter-end]

Eine Liste von ids, die für die Autovervollständigung von **@mentions** verwendet wird. Nützlich, wenn Sie verhindern wollen, dass Benutzer markiert werden, wenn sie keine gemeinsamen Gruppen haben.

Wenn angegeben, werden in der Autovervollständigung nach Eingabe des Zeichens `@` nur Benutzer aus anderen Gruppen angezeigt.

[code-example-start config = {mentionGroupIds: ['yxZAhjzda', 'QT19nXbqB']}; linesToHighlight = [6, 7, 8, 9]; title = 'Limit Groups for Mentions'; code-example-end]

---