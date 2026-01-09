---
По подразумеваној поставци, FastComments ће приказати име корисника онако како га је унео, или како нам је прослеђено преко SSO.

Међутим, може бити пожељно маскирати или приказати име корисника на другачији начин. На пример, ако је име корисника Allen Rex, можда желите да прикажете само "Allen R.".

Ово се може урадити без кода у корисничком интерфејсу за прилагођавање виџета, у подешавању под називом `Commenter Name Format`:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.commenter-name-format select'; selector = '.commenter-name-format'; title='Change Name Format' app-screenshot-end]

Доступни формати су:

- Capitalize (прикажи Example User као Example User)
- Last Initial (прикажи Example User као Example U.)
- All Initials (прикажи Example User као E. U.)
- Show "Anonymous"

Ефекат промене је одмах видљив. Корисници ће и даље видети своје пуно корисничко име на врху области коментара за себе, али ће њихови коментари приказивати модификовано корисничко име.

Корисничка имена се маскирају на серверу ради заштите корисника.

---