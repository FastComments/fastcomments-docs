[related-parameter-start name = 'collapseReplies'; type = 'boolean'; related-parameter-end]

Som standard vises svar på top‑niveau kommentarer.

Dette kan konfigureres, så brugeren skal klikke på "Show Replies" på top‑niveau kommentarer for at se svarene.

[code-example-start config = {collapseReplies: true}; linesToHighlight = [6]; title = 'Collapse Replies to Top Level Comments'; code-example-end]

Dette kan tilpasses uden kode på widget‑tilpasningssiden:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.collapse-replies'; alt='Mulighed for at skjule svar i widget‑tilpasnings‑UI\'et, som gemmer underkommentarer bag et "Show Replies"-link'; title='Skjul svar' app-screenshot-end]

Denne indstilling påvirker ikke antallet af top‑niveau kommentarer, der indlæses initialt. Hvis du har én top‑niveau kommentar og 29 svar, vil du med denne indstilling:

- Se top‑niveau kommentaren.
- Se "Show Replies" (29) under denne kommentar.

Hvis du ønsker at vise alle top‑niveau kommentarer i kombination med denne mulighed, skal du sætte [starting page to -1](#starting-page).