[related-parameter-start name = 'useShowCommentsToggle'; type = 'boolean'; related-parameter-end]

Som standard vil FastComments gengive kommentarfeltet og kommentartråden på samme tid. For at spare noget lodret plads,
vil den også skjule eventuelle andre påkrævede felter, indtil widget'en interageres med.

Dog kan kommentarswidget'en skjules bag en knap, for eksempel:

[app-screenshot-start width=700; url=`https://fastcomments.com/embed?config=%7B%22tenantId%22%3A%22L177BUDVvSe%22%2C%22useShowCommentsToggle%22%3A%22true%22%2C%22urlId%22%3A%22file%253A%252F%252F%252FC%253A%252FUsers%252Fwinrid%252FAppData%252FRoaming%252FJetBrains%252FWebStorm2021.2%252Fscratches%252Fscratch_295.html%22%2C%22url%22%3A%22file%253A%252F%252F%252FC%253A%252FUsers%252Fwinrid%252FAppData%252FRoaming%252FJetBrains%252FWebStorm2021.2%252Fscratches%252Fscratch_295.html%22%2C%22pageTitle%22%3A%22%22%2C%22instanceId%22%3A%220.1281898364813452.1655790389169%22%7D&wId=comment-ui-v2`; selector = '.fast-comments'; delay=2000; alt='Kommentarswidget kollapset bag en knap, der viser kommentarantallet, indtil en læser klikker på den'; title='Klik for at vise kommentarer' app-screenshot-end]

Knapperne bruger forskellig oversat tekst afhængigt af, om kommentarerne i øjeblikket er vist eller ej. Hvis kommentarerne er skjulte, bruges `translations.SHOW_COMMENTS_BUTTON_TEXT`. Hvis
kommentarerne er vist, bruges `translations.HIDE_COMMENTS_BUTTON_TEXT`. Oversættelserne kan indeholde teksten `[count]`, som vil
blive erstattet af det lokaliserede antal.

[code-example-start config = {useShowCommentsToggle: true}; linesToHighlight = [6]; title = 'Klik for at vise eller skjule kommentarer'; code-example-end]

Dette er designet til at erstatte `hideCommentsUnderCountTextFormat`-konfigurationen.

Antallet opdateres live med kommentartråden. Knapperne vises ikke, hvis der ingen kommentarer er.

Dette kan aktiveres uden kode ved at oprette en tilpasningsregel og aktivere "Klik for at vise kommentarer":

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.click-to-show-comments']; selector = '.click-to-show-comments'; alt='Klik for at vise kommentarer afkrydsningsfelt markeret i en tilpasningsregel på widgetens tilpasningsside'; title='Aktiver Klik for at vise kommentarer' app-screenshot-end]

---