[related-parameter-start name = 'disableUnverifiedLabel'; type = 'boolean'; related-parameter-end]

Према подразумеваним поставкама, FastComments ће приказати ознаку "Unverified Comment" за коментаре које је оставио корисник који
има непотврђену сесију прегледача. Прочитајте више о непотврђеном коментарисању [here](https://docs.fastcomments.com/guide-comment-vote-verification.html).

[code-example-start config = {disableUnverifiedLabel: true}; linesToHighlight = [6]; title = 'Disable The Unverified Label'; code-example-end]

Поред тога, ова функција се може користити, без писања кода, у интерфејсу за прилагођавање:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-unverified-comment-label']; selector = '.disable-unverified-comment-label'; title='Disable The Unverified Label' app-screenshot-end]