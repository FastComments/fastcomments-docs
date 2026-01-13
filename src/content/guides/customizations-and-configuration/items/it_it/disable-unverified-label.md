[related-parameter-start name = 'disableUnverifiedLabel'; type = 'boolean'; related-parameter-end]

Per impostazione predefinita, FastComments mostrerà un'etichetta "Commento non verificato" per i commenti lasciati per un utente che
ha una sessione del browser non verificata. Per saperne di più sui commenti non verificati [qui](https://docs.fastcomments.com/guide-comment-vote-verification.html).

[code-example-start config = {disableUnverifiedLabel: true}; linesToHighlight = [6]; title = 'Disable The Unverified Label'; code-example-end]

Inoltre, questa funzionalità può essere utilizzata, senza scrivere codice, nell'interfaccia di personalizzazione:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-unverified-comment-label']; selector = '.disable-unverified-comment-label'; title='Disable The Unverified Label' app-screenshot-end]