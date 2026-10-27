[related-parameter-start name = 'translations'; type = 'Record<string, string>'; related-parameter-end]

Con FastComments, tutto il testo nel widget dei commenti è personalizzabile.

Puoi sovrascrivere un singolo pezzo di testo, come il pulsante di invio, o tutto il testo nell'intero widget dei commenti.

Per impostazione predefinita, il testo nel widget dei commenti viene tradotto in base alla lingua dell'utente. Tuttavia, possiamo sovrascrivere il testo, se siamo certi che la nostra base di utenti utilizzi la stessa lingua/locale, ad esempio:

[code-example-start config = {translations: { SUBMIT_REPLY: 'Reply' } }; linesToHighlight = [6, 7, 8]; title = 'Custom Text'; code-example-end]

Tutte le traduzioni personalizzabili possono essere trovate <a href="https://fastcomments.com/auth/my-account/get-acct-code#translations" target="_blank">qui</a> nella scheda "opzioni avanzate".

Tuttavia, c'è un modo più semplice, tramite l'interfaccia di personalizzazione del widget. Lì, possiamo semplicemente trovare il testo che appare nel widget di commento nella locale EN_US e specificare una sostituzione.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-custom-text-option', '#custom-text-options .dropdown-btn', '.dropdown-items a[data-value="JUST_NOW"]']; selector = '#custom-text-options'; alt='Pannello di testo personalizzato con una stringa del widget selezionata dal menu a discesa e un campo di testo di sostituzione'; title='Testo personalizzato' app-screenshot-end]

Tutte le sovrascritture delle traduzioni attualmente influenzano tutte le lingue.

Questo copre il testo del widget stesso. Per tradurre i commenti scritti dai tuoi visitatori, vedi [Traduzione dei commenti](#comment-translation).