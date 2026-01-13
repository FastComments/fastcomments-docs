[related-parameter-start name = 'mentionGroupIds'; type = 'Array<string>'; related-parameter-end]

Списък с идентификатори за използване при автоматично допълване на **@mentions**. Полезно, когато искате да предотвратите маркирането на потребители, които нямат пресичащи се групи.

Когато е зададено, в автоматичното допълване ще се показват само потребители от други групи след въвеждане на символа `@`.

[code-example-start config = {mentionGroupIds: ['yxZAhjzda', 'QT19nXbqB']}; linesToHighlight = [6, 7, 8, 9]; title = 'Limit Groups for Mentions'; code-example-end]