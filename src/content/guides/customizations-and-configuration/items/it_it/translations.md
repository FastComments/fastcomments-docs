[related-parameter-start name = 'translations'; type = 'Record<string, string>'; related-parameter-end]

Con FastComments, tutto il testo nel widget dei commenti è personalizzabile.

È possibile sovrascrivere un singolo elemento di testo, come il pulsante di invio, o tutto il testo dell'intero widget dei commenti.

Per impostazione predefinita, il testo nel widget dei commenti viene tradotto in base alla locale dell'utente. Tuttavia, possiamo sovrascrivere il testo, se siamo sicuri
che la nostra base di utenti utilizzi la stessa localizzazione/lingua, ad esempio:

[code-example-start config = {translations: { SUBMIT_REPLY: 'Reply' } }; linesToHighlight = [6, 7, 8]; title = 'Custom Text'; code-example-end]

Tutte le traduzioni personalizzabili si trovano <a href="https://fastcomments.com/auth/my-account/get-acct-code#translations" target="_blank">qui</a> nella scheda "opzioni avanzate".

Tuttavia, esiste un modo più semplice, tramite l'interfaccia di personalizzazione del widget. Lì, possiamo semplicemente trovare il testo che appare nel widget dei commenti nella localizzazione EN_US, e specificare
una sostituzione.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-custom-text-option', '#custom-text-options .dropdown-btn', '.dropdown-items a[data-value="JUST_NOW"]']; selector = '#custom-text-options'; title='Custom Text' app-screenshot-end]

Al momento, tutte le sovrascritture delle traduzioni interessano tutte le localizzazioni.

---