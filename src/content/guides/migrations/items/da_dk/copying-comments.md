Hvis data skal flyttes rundt, tilbyder FastComments et selvbetjeningsværktøj til at flytte kommentarer mellem sider og artikler.

Sådan ser formularen til kopiering af kommentarer ud:

[app-screenshot-start url='/auth/my-account/manage-data/copy-comments'; selector = '.account-block'; alt='Formular til kopiering af kommentarer med felterne From URL ID samt To URL ID og URL'; title='Formularen til kopiering af kommentarer' app-screenshot-end]

### Udfyldning af "From"-felterne

For at beslutte, hvor kommentarerne skal flyttes fra, skal vi blot kende kildens `URL ID`.

Hvis du ikke sender en værdi for `urlId` i kommentar-widgettens konfiguration, vil dette være en "ren" version af sidens URL.

Du kan se, hvilke værdier dine kommentarer har for `URL ID`, ved at eksportere dem.

### Udfyldning af "To"-felterne

For at beslutte, hvor kommentarerne skal flyttes til, skal vi kende mål-`URL ID` og `URL`.

`URL ID` vil være den "bucket", som kommentaren havner i. `URL`-feltet bruges, så du kan navigere direkte til kommentaren fra e-mails og moderationsværktøjer.

#### WordPress

Hvis du bruger WordPress, skal du for eksempel indtaste Article IDs i To/From `URL ID`-felterne i migreringsværktøjet i stedet for en URL.