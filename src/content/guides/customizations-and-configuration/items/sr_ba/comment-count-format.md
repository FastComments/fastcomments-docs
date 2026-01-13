---
[related-parameter-start name = 'commentCountFormat'; type = 'string'; related-parameter-end]

Број коментара који је приказан на врху видџета за коментаре може бити прилагођен.

Ово се може заменити било којим низом, а вредност **[count]** биће замењена стварним бројем, локализованим за корисника.

[code-example-start config = {commentCountFormat: "There are [count] comments."}; linesToHighlight = [6]; title = 'Customizing The Comment Count Text'; code-example-end]

Ово се може прилагодити без кода, на страници за прилагођавање видџета:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.comment-count'; title='Customizing The Comment Count Text' app-screenshot-end]


---