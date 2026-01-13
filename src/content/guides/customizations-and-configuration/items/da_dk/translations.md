[related-parameter-start name = 'translations'; type = 'Record<string, string>'; related-parameter-end]

Med FastComments kan al tekst i kommentar-widgeten tilpasses.

Du kan tilsidesætte et enkelt tekststykke, som f.eks. send-knappen, eller al tekst i hele kommentar-widgeten.

Som standard bliver teksten i kommentar-widgeten oversat baseret på brugerens lokalitet. Men vi kan tilsidesætte teksten, hvis vi er sikre på,
at vores brugerbase bruger samme lokalitet/sprog, for eksempel:

[code-example-start config = {translations: { SUBMIT_REPLY: 'Reply' } }; linesToHighlight = [6, 7, 8]; title = 'Custom Text'; code-example-end]

Alle tilpasningsbare oversættelser findes <a href="https://fastcomments.com/auth/my-account/get-acct-code#translations" target="_blank">her</a> under fanen "advanced options".

Der er dog en nemmere måde via widget-tilpasningsbrugergrænsefladen. Der kan vi blot finde den tekst, der vises i kommenterings-widgeten i EN_US-lokalet, og angive
en erstatning.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-custom-text-option', '#custom-text-options .dropdown-btn', '.dropdown-items a[data-value="JUST_NOW"]']; selector = '#custom-text-options'; title='Custom Text' app-screenshot-end]

Alle overskrivninger af oversættelser gælder i øjeblikket for alle lokaliteter.