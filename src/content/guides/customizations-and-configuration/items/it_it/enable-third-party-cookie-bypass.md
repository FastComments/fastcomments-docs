[related-parameter-start name = 'enableThirdPartyCookieBypass'; type = 'boolean'; related-parameter-end]

Per l'autenticazione, FastComments dipende dal fatto che i cookie di terze parti siano abilitati nel tuo browser. Senza di essi, gli utenti dovranno sempre
lasciare la loro email per commentare (a meno che il campo email sia nascosto), e i loro commenti saranno sempre mostrati come non verificati (per impostazione predefinita).

Per aggirare questo, puoi abilitare il bypass dei cookie di terze parti. 

Quando questa impostazione è abilitata, causerà un piccolo popup che mostra un messaggio indicando che l'utente sta effettuando il login. Questo popup
compare ogni volta che l'utente interagisce con il widget dei commenti; per esempio, se lascia un commento.

Possiamo fare questo nel codice impostando il flag **enableThirdPartyCookieBypass** a true:

[code-example-start config = {enableThirdPartyCookieBypass: true}; linesToHighlight = [6]; title = 'Abilitazione del bypass dei cookie di terze parti'; code-example-end]

Possiamo anche configurarlo tramite l'interfaccia di personalizzazione del widget, sotto `Enable Third-Party Cookie Popup`:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.enable-third-party-cookie-bypass'; clickSelectors = ['.enable-third-party-cookie-bypass']; alt='Pagina di personalizzazione del widget con la casella Enable Third-Party Cookie Popup selezionata'; title='Abilitazione del bypass dei cookie di terze parti' app-screenshot-end]

---