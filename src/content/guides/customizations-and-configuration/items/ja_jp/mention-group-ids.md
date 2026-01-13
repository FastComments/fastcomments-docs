[related-parameter-start name = 'mentionGroupIds'; type = 'Array<string>'; related-parameter-end]

オートコンプリートの**@mentions**で使用するIDの一覧。ユーザー同士のグループが交差していない場合にタグ付けされないようにしたいときに便利です。

指定すると、`@`文字を入力したときに、他のグループに属するユーザーのみがオートコンプリートに表示されます。

[code-example-start config = {mentionGroupIds: ['yxZAhjzda', 'QT19nXbqB']}; linesToHighlight = [6, 7, 8, 9]; title = 'Limit Groups for Mentions'; code-example-end]