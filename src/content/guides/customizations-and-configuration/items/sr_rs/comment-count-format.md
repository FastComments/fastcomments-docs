[related-parameter-start name = 'commentCountFormat'; type = 'string'; related-parameter-end]

Број коментара приказан на врху виџета за коментаре може бити прилагођен.

Може бити замењен било којим текстом, а вредност **[count]** биће замењена вредношћу броја, прилагођеном кориснику.

[code-example-start config = {commentCountFormat: "There are [count] comments."}; linesToHighlight = [6]; title = 'Прилагођавање текста броја коментара'; code-example-end]

Ово може бити прилагођено без кода, на страници за прилагођавање виџета:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.comment-count'; alt='Поље за текст броја коментара на страници за прилагођавање виџета, где се [count] замењује са тренутним укупним бројем'; title='Прилагођавање текста броја коментара' app-screenshot-end]