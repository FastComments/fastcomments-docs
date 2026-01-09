[related-parameter-start name = 'enableThirdPartyCookieBypass'; type = 'boolean'; related-parameter-end]

Per l'autenticazione, FastComments dipende dall'abilitazione dei cookie di terze parti nel tuo browser. Senza di essi, gli utenti dovranno sempre
lasciare la loro email per commentare (a meno che il campo email non sia nascosto), e i loro commenti verranno sempre mostrati come non verificati (per impostazione predefinita).

Per aggirare questo, puoi abilitare il bypass dei cookie di terze parti. 

Quando questa impostazione è abilitata, apparirà un piccolo popup che mostra un messaggio che indica che l'utente viene autenticato. Questo popup
viene mostrato ogni volta che l'utente interagisce con il widget dei commenti; ad esempio, quando lascia un commento.

Possiamo farlo nel codice impostando il flag **enableThirdPartyCookieBypass** a true:

[code-example-start config = {enableThirdPartyCookieBypass: true}; linesToHighlight = [6]; title = 'Enabling Third-Party Cookie Bypass'; code-example-end]

Possiamo anche configurarlo tramite l'interfaccia di personalizzazione del widget, sotto `Enable Third-Party Cookie Popup`:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.enable-third-party-cookie-bypass'; clickSelectors = ['.enable-third-party-cookie-bypass']; title='Enabling Third-Party Cookie Bypass' app-screenshot-end]

---