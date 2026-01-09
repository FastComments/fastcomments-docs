[related-parameter-start name = 'useSingleLineCommentInput'; type = 'boolean'; related-parameter-end]

По подразумјеву, FastComments ће дозволити кориснику да унесе коментар са онолико редова колико жели, у оквиру подразумјеваног ограничења броја знакова.

Међутим, може бити пожељно ограничити корисника да унесе само један ред текста. Неке примјере употребе укључују онлајн надметање или уживо ћаскање, за која се може користити FastComments.

Овако омогућавамо флаг **useSingleLineCommentInput**:

[code-example-start config = {useSingleLineCommentInput: true}; linesToHighlight = [6]; title = 'Enable Single-Line Comment Input'; code-example-end]

Ово се може урадити и без кода. На страници за прилагођавање виџета, погледајте одељак "Enable Single-Line Comment Input".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.single-line-comment-input'; selector = '.single-line-comment-input'; title='Enable Single-Line Comment Input' app-screenshot-end]

Имајте на уму да се коментари за сваку страницу и сваки смер сортирања предрачунавају, тако да све опције сортирања имају исте перформансе.