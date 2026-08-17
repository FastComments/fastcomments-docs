[related-parameter-start name = 'commentCountFormat'; type =string; related-parameter-end]

Број коментара приказан на врху виџета за коментаре може се прилагодити.

Ово се може заменити било којим низом знакова, а вредност **[count]** ће бити замењена вредношћу броја, локализованом за корисника.

[code-example-start config = {commentCountFormat: "There are [count] comments."}; linesToHighlight = [6]; title = 'Прилагођавање текста броја коментара'; code-example-end]

Ово се може прилагодити без кода, на страници за прилагођавање виџета:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.comment-count'; alt='Поље за текст броја коментара на страници за прилагођавање виџета, где се [count] замењује живим укупним бројем'; title='Прилагођавање текста броја коментара' app-screenshot-end]