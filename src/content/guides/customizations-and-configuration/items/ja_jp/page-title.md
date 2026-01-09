[related-parameter-start name = 'pageTitle'; type = 'string'; related-parameter-end]

現在のページタイトルは指定された **urlId** に関連付けられ、モデレーションツールで使用するために保存されます。

デフォルトでは、これは **document.title** から取得されます。

必要に応じて、独自のページタイトルを次のように指定できます：

[code-example-start config = {pageTitle: "Article 42"}; linesToHighlight = [6]; title = 'Specifying The Page Title'; code-example-end]