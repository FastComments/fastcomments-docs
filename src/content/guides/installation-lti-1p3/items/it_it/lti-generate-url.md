#### Vai alla configurazione LTI 1.3

Accedi a FastComments e vai a <a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">la tua pagina di configurazione LTI 1.3</a>.

Se il tuo account non ha ancora accesso a LTI, vedrai "LTI non abilitato per questo account" - contatta il supporto per abilitarlo sul tuo piano.

#### Scegli una piattaforma (opzionale)

Sotto **Genera un URL di registrazione dinamico**, usa il menu a discesa **Piattaforma** per indicare a FastComments a quale LMS ti stai collegando:

- D2L Brightspace
- Moodle
- Blackboard Learn
- Sakai
- Schoology
- Altra piattaforma LTI 1.3

Puoi anche lasciarlo su **Rilevamento automatico**. La piattaforma viene letta dall'openid-configuration del tuo LMS durante la registrazione; il menu a discesa imposta solo l'etichetta di visualizzazione per la configurazione risultante.

#### Genera l'URL

Fai clic su **Genera URL**. FastComments crea un token di registrazione monouso e ti mostra un URL che appare come:

`https://fastcomments.com/lti/v1p3/register/<long-token>`

Copialo. Questo URL:

- È **monouso** - una volta che il tuo LMS lo richiama con successo, il token viene consumato.
- Scade dopo **30 minuti** se non viene usato.
- Deve essere mantenuto privato - chiunque abbia l'URL può registrare uno strumento per il tuo tenant entro quei 30 minuti.

#### Configurazioni esistenti

Una volta che una registrazione è completata con successo, la nuova configurazione viene visualizzata nella tabella **Configurazioni esistenti** sulla stessa pagina, con Piattaforma, Emittente, Client ID e Stato. Puoi eliminare le configurazioni da questa tabella se mai dovessi annullare la registrazione.