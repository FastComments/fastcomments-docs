[related-parameter-start name = 'enableCommenterLinks'; type = 'boolean'; related-parameter-end]

Som standard vil FastComments kun bede brugeren om deres kommentar, deres brugernavn og deres e‑mail.

Men i nogle situationer kan du ønske, at brugeren efterlader et link til deres egen blog eller hjemmeside.

Vi kan aktivere visning af et ekstra input‑felt til at indtaste brugerens website‑URL ved at sætte **enableCommenterLinks**‑flaget til true:

[code-example-start config = {enableCommenterLinks: true}; linesToHighlight = [6]; title = 'Aktivering af kommentatorlinks'; code-example-end]

Når den pågældende URL er angivet, vil brugerens konto blive opdateret, og deres brugernavn på alle tidligere og fremtidige kommentarer vil linke til denne URL.

Dette kan tilpasses uden kode på widget‑tilpasningssiden:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.click-to-show-comments', '.commenter-links']; selector = '.commenter-links'; alt='Widget-tilpasningsside med kommentatorlink‑afkrydsningsfeltet markeret for at tilføje et website‑URL‑felt til kommentarfeltet'; title='Aktivering af kommentatorlinks' app-screenshot-end]

---