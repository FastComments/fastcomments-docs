[related-parameter-start name = 'countAll'; type = 'boolean'; related-parameter-end]

コメントウィジェットの上部に表示されるコメント数は、ページや記事自体への直接の返信である「トップレベル」のコメントのみを表示するか、ネストされた**すべて**のコメントの数を表示するかを選択できます。

デフォルトでは、これは `true` です。つまり、後者のすべてのコメントの数を表示します。古いバージョンのコメントウィジェットでは、デフォルト値は `false` です。

**countAll** フラグを true に設定することで、ネストされた**すべて**のコメントの数を表示するように動作を変更できます。

[code-example-start config = {countAll: true}; linesToHighlight = [6]; title = 'すべてのコメントをカウントする'; code-example-end]

トップレベルのコメントのみをカウントしたい場合は、フラグを false に設定します。

[code-example-start config = {countAll: false}; linesToHighlight = [6]; title = 'トップレベルのコメントをカウントする'; code-example-end]

現在、この設定はコードを変更せずにカスタマイズすることはできません。