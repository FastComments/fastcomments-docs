FastComments autentica le richieste al tuo account per verificare che provengano dal tuo sito. Questo è il motivo per cui
dobbiamo sapere su quale sito, o quali siti, vuoi installare FastComments.

FastComments supporta l'autenticazione tramite dominio, nonché tramite sottodomini.

Prendiamo il sito `https://example.com`. In questo caso, "`example.com`" è il dominio. `example.com` supporta sia `example.com`, sia `www.example.com`. Chiameremo "www" il "sottodominio".

Per esempio:

- Per consentire solo `blog.example.com`:
  - Aggiungi `blog.example.com` ai tuoi domini.
- Per consentire `www.example.com`, `somesite.example.com`, e `example.com`:
  - Aggiungi `example.com` ai tuoi domini.
  - Questo viene fatturato come avere **un dominio** associato al tuo account.
- Ora puoi aggiungere sottodomini wildcard, per esempio *myname.vercel.app. 
  - Questo viene fatturato come avere **un dominio** associato al tuo account.

Se stessi usando una piattaforma di blogging e ti fosse stato assegnato un sottodominio, vorresti
aggiungere **l'intero dominio, compreso il sottodominio** al tuo account, per esempio: `cats.blogger.com`.

Possiamo aggiungere domini al nostro account visitando la pagina `My Domains` e cliccando `Add a Domain` in fondo:

[app-screenshot-start url='/auth/my-account/configure-domains'; selector = '.content'; title='The My Domains Page' app-screenshot-end]

Durante il periodo di prova, **i domini vengono aggiunti automaticamente al tuo account** quando le richieste provengono da detti domini. Tuttavia,
dopo questo periodo devono essere aggiunti esplicitamente per motivi di sicurezza. Dovresti ricevere un'email quando si verifica questo comportamento automatizzato.

Non devi **aggiungere** `localhost` per lo sviluppo locale - è consentito di default.

#### Tramite l'API

I domini possono anche essere aggiunti e configurati [via the DomainConfigs API](/guide-api.html#domain-config-structure).