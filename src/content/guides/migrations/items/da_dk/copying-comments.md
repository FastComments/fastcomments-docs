Hvis data skal flyttes, stiller FastComments et selvbetjeningsværktøj til rådighed til at flytte kommentarer
mellem sider og artikler.

Sådan ser formularen til kopiering af kommentarer ud:

[app-screenshot-start url='/auth/my-account/manage-data/copy-comments'; selector = '.account-block'; title='The Copy Comment Form' app-screenshot-end]

### Udfyldelse af "Fra"-felterne

For at beslutte, hvor kommentarerne skal flyttes fra, behøver vi blot at kende kildens `URL ID`.

Hvis du ikke sender en værdi for `urlId` i kommentar-widgetens konfiguration, vil dette være en "ren" version af sidens URL.

Du kan se, hvilke værdier dine kommentarer har for `URL ID` ved at eksportere dem.

### Udfyldelse af "Til"-felterne

For at beslutte, hvor kommentarerne skal flyttes til, har vi brug for målets `URL ID` og `URL`.

`URL ID` vil være den bucket, som kommentaren placeres i. Feltet `URL` bruges, så du kan navigere direkte
til kommentaren fra e-mails og moderationsværktøjer.

#### WordPress

Hvis du bruger WordPress, ville du for eksempel indtaste artikel-ID'erne i Til/Fra `URL ID`-felterne i migrationsværktøjet,
i stedet for en URL.