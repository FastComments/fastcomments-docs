[related-parameter-start name = 'countAll'; type = 'boolean'; related-parameter-end]

コメントウィジェットの上部に表示されるコメント数は、すべての「トップレベル」コメントを表示することもできます。すなわちページや記事自体に直接返信された返信を表示することも、
または**すべて**のネストされたコメントの総数を表示することもできます。

By default, this is `true` - it is a count of the latter - all comments. コメントウィジェットの古いバージョンでは、
デフォルト値は `false` です。

We can change the behavior, so that it is a count of **all** nested comments by setting the **countAll** flag to true.

[code-example-start config = {countAll: true}; linesToHighlight = [6]; title = 'Counting All Comments'; code-example-end]

If we wanted the count to reflect only the top level comments, we set the flag to false.

[code-example-start config = {countAll: false}; linesToHighlight = [6]; title = 'Counting Top Level Comments'; code-example-end]

This currently cannot be customized without code changes.