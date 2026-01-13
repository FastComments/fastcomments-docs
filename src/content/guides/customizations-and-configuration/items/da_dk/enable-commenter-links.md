[related-parameter-start name = 'enableCommenterLinks'; type = 'boolean'; related-parameter-end]

Som standard vil FastComments kun bede brugeren om deres kommentar, deres brugernavn og deres e-mail.

I nogle situationer vil du dog måske have, at brugeren efterlader et link til deres egen blog eller hjemmeside.

Vi kan aktivere visningen af et ekstra inputfelt til brugerens websteds-URL ved at sætte flaget **enableCommenterLinks** til true:

[code-example-start config = {enableCommenterLinks: true}; linesToHighlight = [6]; title = 'Enabling Commenter Links'; code-example-end]

Når den nævnte URL angives, opdateres brugerens konto, og alle deres brugernavne på tidligere og fremtidige kommentarer vil linke til denne URL.

Dette kan tilpasses uden kode på siden til widget-tilpasning:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.click-to-show-comments', '.commenter-links']; selector = '.commenter-links'; title='Enabling Commenter Links' app-screenshot-end]

---