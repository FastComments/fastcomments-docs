По подразумеваној поставци, FastComments ће приказати име корисника како су га унели, или како нам је прослеђено преко SSO.

Међутим, може бити пожељно прикрити или приказати име корисника на другачији начин. На пример, ако је име корисника Allen Rex, можда желите да прикажете само "Allen R.".

Ово се може урадити без кода у Widget Customization UI, под поставком која се зове `Commenter Name Format`:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.commenter-name-format select'; selector = '.commenter-name-format'; title='Change Name Format' app-screenshot-end]

Доступни формати су:

- Име са великим почетним словом (прикажи Example User као Example User)
- Иницијал презимена (прикажи Example User као Example U.)
- Сви иницијали (прикажи Example User као E. U.)
- Прикажи "Anonymous"

Утицај промене је непосредан. Корисници ће и даље видети своје пуно корисничко име на врху подручја за коментаре, за себе, али ће њихови коментари показивати измењено корисничко име.

Корисничка имена се маскирају на серверској страни ради заштите корисника.