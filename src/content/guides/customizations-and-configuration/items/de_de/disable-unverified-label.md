[related-parameter-start name = 'disableUnverifiedLabel'; type = 'boolean'; related-parameter-end]

Standardmäßig zeigt FastComments ein "Unbestätigter Kommentar" Label für Kommentare an, die für einen Nutzer hinterlassen wurden, der eine nicht verifizierte Browsersitzung hat. Mehr zum unbestätigten Kommentieren lesen Sie [hier](https://docs.fastcomments.com/guide-comment-vote-verification.html).

[code-example-start config = {disableUnverifiedLabel: true}; linesToHighlight = [6]; title = 'Disable The Unverified Label'; code-example-end]

Zusätzlich kann diese Funktion ohne Programmieraufwand in der Customization UI verwendet werden:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-unverified-comment-label']; selector = '.disable-unverified-comment-label'; title='Disable The Unverified Label' app-screenshot-end]

---