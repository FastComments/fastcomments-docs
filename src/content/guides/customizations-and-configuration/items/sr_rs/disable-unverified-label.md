[related-parameter-start name = 'disableUnverifiedLabel'; type = 'boolean'; related-parameter-end]

По подразумеваној поставци, FastComments ће приказати ознаку „Неовјерени коментар“ за коментаре који су остављени за корисника који има неовјерену сесију прегледача. Прочитајте више о неовјереном коментарисању [овде](https://docs.fastcomments.com/guide-comment-vote-verification.html).

[code-example-start config = {disableUnverifiedLabel: true}; linesToHighlight = [6]; title = 'Disable The Unverified Label'; code-example-end]

Поред тога, ова функција се може користити, без писања кода, у Customization UI:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-unverified-comment-label']; selector = '.disable-unverified-comment-label'; title='Disable The Unverified Label' app-screenshot-end]

---