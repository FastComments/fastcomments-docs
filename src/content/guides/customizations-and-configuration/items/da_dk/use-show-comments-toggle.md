[related-parameter-start name = 'useShowCommentsToggle'; type = 'boolean'; related-parameter-end]

Som standard gengiver FastComments kommentarboksen og kommentartråden samtidig. For at spare noget lodret plads,
vil den også skjule alle andre påkrævede felter, indtil der interageres med widgeten.

Kommentarwidgeten kan dog skjules bag en knap, for eksempel:

[app-screenshot-start width=700; url=`https://fastcomments.com/embed?config=%7B%22tenantId%22%3A%22L177BUDVvSe%22%2C%22useShowCommentsToggle%22%3A%22true%22%2C%22urlId%22%3A%22file%253A%252F%252F%252FC%253A%252FUsers%252Fwinrid%252FAppData%252FRoaming%252FJetBrains%252FWebStorm2021.2%252Fscratches%252Fscratch_295.html%22%2C%22url%22%3A%22file%253A%252F%252F%252FC%253A%252FUsers%252Fwinrid%252FAppData%252FRoaming%252FJetBrains%252FWebStorm2021.2%252Fscratches%252Fscratch_295.html%22%2C%22pageTitle%22%3A%22%22%2C%22instanceId%22%3A%220.1281898364813452.1655790389169%22%7D&wId=comment-ui-v2`; selector = '.fast-comments'; delay=2000; title='Click to Show Comments' app-screenshot-end]

Knappen bruger forskellig oversat tekst afhængigt af, om kommentarerne i øjeblikket vises eller ej. Hvis kommentarerne er skjult, bruger den `translations.SHOW_COMMENTS_BUTTON_TEXT`. Hvis kommentarerne vises, bruger den `translations.HIDE_COMMENTS_BUTTON_TEXT`. Oversættelserne kan indeholde teksten `[count]` som vil
blive erstattet af det lokaliserede antal.

[code-example-start config = {useShowCommentsToggle: true}; linesToHighlight = [6]; title = 'Click to Show or Hide Comments'; code-example-end]

Dette er designet til at erstatte konfigurationen `hideCommentsUnderCountTextFormat`.

Tælleren opdateres live sammen med kommentartråden. Knappen vises ikke, hvis der ikke er nogen kommentarer.

Dette kan aktiveres uden kode ved at oprette en tilpasningsregel og aktivere "Click to Show Comments":

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.click-to-show-comments']; selector = '.click-to-show-comments'; title='Enable Click to Show Comments' app-screenshot-end]