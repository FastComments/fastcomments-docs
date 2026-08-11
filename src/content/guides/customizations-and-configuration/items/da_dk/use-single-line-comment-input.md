[related-parameter-start name = 'useSingleLineCommentInput'; type = 'boolean'; related-parameter-end]

Som standard tillader FastComments brugeren at indtaste en kommentar med så mange linjer, som de ønsker, op til standardtegnbegrænsningen.

Det kan dog være ønskeligt at begrænse brugeren til kun at indtaste en enkelt linje tekst. Nogle eksempler på anvendelsestilfælde inkluderer onlinebudgivning eller live chat, som FastComments kan bruges til.

Vi aktiverer flaget **useSingleLineCommentInput** som følger:

[code-example-start config = {useSingleLineCommentInput: true}; linesToHighlight = [6]; title = 'Enable Single-Line Comment Input'; code-example-end]

Dette kan også gøres uden kode. På widget-tilpasningssiden, se afsnittet "Aktiver enkeltlinje-kommentarinput".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.single-line-comment-input'; selector = '.single-line-comment-input'; alt='Enkeltlinje-kommentarinput afkrydsningsfelt slået til på widget-tilpasningssiden, begrænser input til én linje'; title='Aktiver enkeltlinje-kommentarinput' app-screenshot-end]

Bemærk, at kommentarerne på hver side for hver sorteringsretning er forudberegnet, så alle sorteringsretninger har samme ydeevne.