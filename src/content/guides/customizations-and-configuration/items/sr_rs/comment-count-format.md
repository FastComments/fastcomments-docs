---
[related-parameter-start name = 'commentCountFormat'; type = 'string'; related-parameter-end]

Број коментара који се приказује на врху видгета за коментаре може се прилагодити.

Ово може бити замењено било којим низом, а вредност **[count]** биће замењена бројем коментара, локализованим за корисника.

[code-example-start config = {commentCountFormat: "There are [count] comments."}; linesToHighlight = [6]; title = 'Прилагођавање текста броја коментара'; code-example-end]

Ово се може прилагодити без писања кода, на страници за прилагођавање виджета:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.comment-count'; title='Прилагођавање текста броја коментара' app-screenshot-end]


---