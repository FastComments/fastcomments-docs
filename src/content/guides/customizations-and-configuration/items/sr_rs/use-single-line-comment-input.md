[related-parameter-start name = 'useSingleLineCommentInput'; type = 'boolean'; related-parameter-end]

Подразумевано, FastComments ће дозволити кориснику да уноси коментар у колико год линија жели, до подразумеваног ограничења знакова.

Међутим, може бити пожељно ограничити корисника да уноси само једну линију текста. Неки примери употребе укључују онлајн надметање или живи ћаскање, за које се FastComments може користити.

Омогућавамо заставицу **useSingleLineCommentInput** на следећи начин:

[code-example-start config = {useSingleLineCommentInput: true}; linesToHighlight = [6]; title = 'Омогући унос коментара у једној линији'; code-example-end]

Ово се такође може урадити без кода. На страници за прилагођавање widget-а, погледајте одељак „Enable Single-Line Comment Input“.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.single-line-comment-input'; selector = '.single-line-comment-input'; alt='Поље за унос коментара у једној линији је укључено на страници за прилагођавање widget-а, ограничавајући унос на једну линију'; title='Омогући унос коментара у једној линији' app-screenshot-end]

Имајте на уму да се коментари на свакој страници за сваки смер сортирања предрачунавају, тако да сви смерови сортирања имају исту перформансу.