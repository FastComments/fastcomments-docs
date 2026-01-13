[related-parameter-start name = 'mentionGroupIds'; type = 'Array<string>'; related-parameter-end]

用于 **@mentions** 自动完成 的 ID 列表。当你希望在用户没有交集群组时防止标记他们时非常有用。

当指定后，输入 `@` 字符后，自动完成中只会提供属于其他群组的用户。

[code-example-start config = {mentionGroupIds: ['yxZAhjzda', 'QT19nXbqB']}; linesToHighlight = [6, 7, 8, 9]; title = 'Limit Groups for Mentions'; code-example-end]

---