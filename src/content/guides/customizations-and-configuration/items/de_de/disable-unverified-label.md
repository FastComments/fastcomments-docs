[related-parameter-start name = 'disableUnverifiedLabel'; type = 'boolean'; related-parameter-end]

Standardmäßig zeigt FastComments ein "Unverified Comment"-Label für Kommentare an, die für einen Benutzer hinterlassen wurden, der eine nicht verifizierte Browsersitzung hat. Weitere Informationen zum Kommentieren ohne Verifizierung finden Sie [hier](https://docs.fastcomments.com/guide-comment-vote-verification.html).

[code-example-start config = {disableUnverifiedLabel: true}; linesToHighlight = [6]; title = 'Disable The Unverified Label'; code-example-end]

Zusätzlich kann diese Funktion, ohne Code zu schreiben, in der Customization UI verwendet werden:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-unverified-comment-label']; selector = '.disable-unverified-comment-label'; alt='Widget-Anpassungsseite mit dem aktivierten Kontrollkästchen "Disable Unverified Comment Label"'; title='Deaktivieren des Unverified-Labels' app-screenshot-end]