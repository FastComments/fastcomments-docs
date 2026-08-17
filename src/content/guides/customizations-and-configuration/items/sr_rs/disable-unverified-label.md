---
[related-parameter-start name = 'disableUnverifiedLabel'; type = 'boolean'; related-parameter-end]

Подразумевано, FastComments ће приказати ознаку „Неоверификован коментар“ за коментаре који су остављени за корисника који има неоверификовану сесију прегледача. Прочитајте више о неоверификованом коментарисању [овде](https://docs.fastcomments.com/guide-comment-vote-verification.html).

[code-example-start config = {disableUnverifiedLabel: true}; linesToHighlight = [6]; title = 'Disable The Unverified Label'; code-example-end]

Додатно, ова функција се може користити, без писања кода, у корисничком интерфејсу за прилагођавање:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-unverified-comment-label']; selector = '.disable-unverified-comment-label'; alt='Страница за прилагођавање виџета са чекираним пољем за онемогућавање ознаке Неоверификованог коментара'; title='Онемогући ознаку Неоверификованог коментара' app-screenshot-end]