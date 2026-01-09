[related-parameter-start name = 'mentionGroupIds'; type = 'Array<string>'; related-parameter-end]

**@mentions** 자동 완성에 사용할 id 목록입니다. 교집합 그룹이 없는 사용자에게 태그되는 것을 방지하려는 경우에 유용합니다.

지정하면 `@` 문자를 입력한 후 자동 완성에는 다른 그룹에 속한 사용자만 제공됩니다.

[code-example-start config = {mentionGroupIds: ['yxZAhjzda', 'QT19nXbqB']}; linesToHighlight = [6, 7, 8, 9]; title = 'Limit Groups for Mentions'; code-example-end]

---