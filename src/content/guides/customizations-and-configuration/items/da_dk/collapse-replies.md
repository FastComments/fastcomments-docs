[related-parameter-start name = 'collapseReplies'; type = 'boolean'; related-parameter-end]

Som standard vises svar på overordnede kommentarer.

Dette kan konfigureres, så brugeren skal klikke "Vis svar" på topniveau-kommentarerne for at se underkommentarerne.

[code-example-start config = {collapseReplies: true}; linesToHighlight = [6]; title = 'Collapse Replies to Top Level Comments'; code-example-end]

Dette kan tilpasses uden kode, på siden til tilpasning af widgeten:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.collapse-replies'; title='Collapse Replies' app-screenshot-end]

Denne indstilling påvirker ikke antallet af topniveau-kommentarer, der indlæses initialt. Hvis du har én topniveau-kommentar og 29 underkommentarer, vil du med denne indstilling slået til:

- Se topniveau-kommentaren.
- Se Vis svar (29) under denne kommentar.

Hvis du ønsker at vise alle topniveau-kommentarer i kombination med denne mulighed, sæt [startside til -1](#starting-page).