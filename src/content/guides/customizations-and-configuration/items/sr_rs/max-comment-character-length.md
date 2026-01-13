[related-parameter-start name = 'maxCommentCharacterLength'; type = 'number'; related-parameter-end]

Максималан број знакова који се могу унети у поље за коментар може се ограничити параметром **maxCommentCharacterLength**.

Подразумевано је 2000.

Ствари као што су URL-адресе слика нису укључене у одређивање дужине.

[code-example-start config = {maxCommentCharacterLength: 500}; linesToHighlight = [6]; title = 'Limit Comment Length'; code-example-end]

Ово се може прилагодити без писања кода, на страници за прилагођавање виџета:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.max-comment-size'; title='Limit Comment Length' app-screenshot-end]