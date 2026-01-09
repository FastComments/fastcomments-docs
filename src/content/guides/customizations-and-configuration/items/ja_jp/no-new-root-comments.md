[related-parameter-start name = 'noNewRootComments'; type = 'boolean'; related-parameter-end]

`noNewRootComments` を `true` に設定すると、ウィジェットはルート返信エリアを非表示にしますが、ユーザーは返信
を子コメントに対して行うことができます。例えば、ページ読み込み時に条件付きでこれを設定し、一部のユーザーのみがトップレベルのコメントを残せるようにすることができます。

[code-example-start config = {noNewRootComments: true}; linesToHighlight = [6]; title = 'Prevent New Root Comments'; code-example-end]

---