[related-parameter-start name = 'mentionGroupIds'; type = 'Array<string>'; related-parameter-end]

一個用於 **@mentions** 自動完成的 ID 列表。當你想要防止在沒有交集群組的使用者被標註時，這會很有用。

指定時，輸入 `@` 字元後，自動完成只會提供其他群組的使用者。

[code-example-start config = {mentionGroupIds: ['yxZAhjzda', 'QT19nXbqB']}; linesToHighlight = [6, 7, 8, 9]; title = 'Limit Groups for Mentions'; code-example-end]

---