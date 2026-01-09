[related-parameter-start name = 'useSingleLineCommentInput'; type = 'boolean'; related-parameter-end]

По подразумевању, FastComments ће дозволити кориснику да унесе коментар који може имати онолико редова колико жели, до подразумеваног ограничења броја карактера.

Међутим, може бити пожељно ограничити корисника да унесе само један ред текста. Неки примјери употребе укључују онлајн надметање или ћаскање уживо, за које се FastComments
може користити.

Флаг **useSingleLineCommentInput** омогућавамо на следећи начин:

[code-example-start config = {useSingleLineCommentInput: true}; linesToHighlight = [6]; title = 'Enable Single-Line Comment Input'; code-example-end]

Ово се такође може урадити и без кода. На страници за прилагођавање виџета, погледајте одељак "Омогућите унос коментара у једном реду".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.single-line-comment-input'; selector = '.single-line-comment-input'; title='Enable Single-Line Comment Input' app-screenshot-end]

Имајте на уму да се коментари на свакој страници за сваки смер сортирања претрачунавају унапред, тако да сви начини сортирања имају исте перформансе.

---