[related-parameter-start name = 'translations'; type = 'Record<string, string>'; related-parameter-end]

Med FastComments kan al tekst i kommentarfunktionen tilpasses.

Du kan tilsidesætte et enkelt stykke tekst, som f.eks. indsendelsesknappen, eller al tekst i hele kommentarfunktionen.

Som standard oversættes teksten i kommentarfunktionen baseret på brugerens locale. Men vi kan tilsidesætte teksten, hvis vi er sikre på, at vores brugerbase bruger den samme locale/sprog, for eksempel:

[code-example-start config = {translations: { SUBMIT_REPLY: 'Reply' } }; linesToHighlight = [6, 7, 8]; title = 'Tilpasset tekst'; code-example-end]

Alle tilpasselige oversættelser kan findes <a href="https://fastcomments.com/auth/my-account/get-acct-code#translations" target="_blank">her</a> under den "avancerede indstillinger" tab.

Der er dog en nemmere måde via widget-tilpasnings‑UI'en. Der kan vi blot finde den tekst, der vises i kommentarfunktionen i EN_US‑locale, og angive en erstatning.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-custom-text-option', '#custom-text-options .dropdown-btn', '.dropdown-items a[data-value="JUST_NOW"]']; selector = '#custom-text-options'; alt='Panel for tilpasset tekst med en widget‑streng valgt fra dropdown‑listen og et erstatningstekstfelt'; title='Tilpasset tekst' app-screenshot-end]

Alle oversættelses‑overrides påvirker i øjeblikket alle locale.