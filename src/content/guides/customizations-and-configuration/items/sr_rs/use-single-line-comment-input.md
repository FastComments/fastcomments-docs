[related-parameter-start name = 'useSingleLineCommentInput'; type = 'boolean'; related-parameter-end]

По подразумевању, FastComments ће дозволити кориснику да унесе коментар са онолико редова колико желе, до подразумеваног ограничења знакова.

Међутим, можда ће бити пожељно ограничити корисника да уноси само један ред текста. Неке примерне употребе укључују онлајн надметање или уживо ћаскање, за које се FastComments може користити.

Флаг **useSingleLineCommentInput** омогућавамо на следећи начин:

[code-example-start config = {useSingleLineCommentInput: true}; linesToHighlight = [6]; title = 'Enable Single-Line Comment Input'; code-example-end]

Ово се може урадити и без кода. На страници за прилагођавање виџета, погледајте одељак "Омогући унос коментара у једном реду".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.single-line-comment-input'; selector = '.single-line-comment-input'; title='Enable Single-Line Comment Input' app-screenshot-end]

Имајте на уму да су коментари на свакој страници за сваки смер сортирања претходно израчунати, тако да све опције сортирања имају исте перформансе.