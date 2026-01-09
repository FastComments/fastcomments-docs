[related-parameter-start name = 'mentionGroupIds'; type = 'Array<string>'; related-parameter-end]

A list of IDs to use for **@mentions** autocomplete. Useful when you want to prevent tagging users when they do not have intersecting groups.

When specified, only users in other groups will be provided in the autocomplete after typing the `@` character.

[code-example-start config = {mentionGroupIds: ['yxZAhjzda', 'QT19nXbqB']}; linesToHighlight = [6, 7, 8, 9]; title = 'Limit Groups for Mentions'; code-example-end]