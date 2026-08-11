FastComments autentica le richieste al tuo account per verificare che provengano dal tuo sito. Per questo  
abbiamo bisogno di sapere quale sito, o quali siti, desideri installare FastComments.

FastComments supporta l'autenticazione tramite dominio, così come i sottodomini.

Prendiamo il sito `https://example.com`. In questo caso, "`example.com`" è il dominio. `example.com` supporta sia `example.com`, sia `www.example.com`. Chiamiamo il "www" il "sottodominio".

Ad esempio:

- Per consentire solo `blog.example.com`:
  - Aggiungi `blog.example.com` ai tuoi domini.
- Per consentire `www.example.com`, `somesite.example.com` e `example.com`:
  - Aggiungi `example.com` ai tuoi domini.
  - Questo viene fatturato come **un dominio** associato al tuo account.
- Ora puoi aggiungere sottodomini wildcard, ad esempio *myname.vercel.app.  
  - Questo viene fatturato come **un dominio** associato al tuo account.

Se stavi usando una piattaforma di blogging e ti è stato fornito un sottodominio, dovresti  
aggiungere il **dominio completo includendo il sottodominio** al tuo account, ad esempio: `cats.blogger.com`.

Possiamo aggiungere domini al nostro account visitando la pagina `My Domains` e facendo clic su `Add a Domain` in fondo:

[app-screenshot-start url='/auth/my-account/configure-domains'; selector = '.content'; alt='Pagina My Domains che elenca i domini sull\'account, con il pulsante Aggiungi un dominio in fondo'; title='La pagina My Domains' app-screenshot-end]

Durante il periodo di prova, **i domini vengono aggiunti automaticamente al tuo account** quando le richieste provengono da tali domini. Tuttavia,  
dopo questo periodo devono essere aggiunti esplicitamente per motivi di sicurezza. Dovresti ricevere un'email quando si verifica questo comportamento automatizzato.

Non è necessario aggiungere `localhost` per lo sviluppo locale – è consentito per impostazione predefinita.

#### Via The API

I domini possono anche essere aggiunti e configurati [via l'API DomainConfigs](/guide-api.html#domain-config-structure).