---
Il plugin supporta tre modalità SSO per integrare gli account utente di Moodle con FastComments.

#### SSO sicuro (Consigliato)

I dati utente vengono firmati lato server usando HMAC-SHA256 con il tuo **API Secret**. Questa è l'opzione più sicura ed è consigliata per l'uso in produzione.

Con SSO sicuro:

- Nomi utente, email e avatar vengono trasmessi in modo sicuro a FastComments.
- Gli amministratori del sito Moodle vengono sincronizzati automaticamente come moderatori di FastComments.
- Gli utenti non possono impersonare altri account.
- Richiede che il **API Secret** sia configurato nelle impostazioni del plugin.

#### SSO semplice

I dati utente (nome, email, avatar) vengono inviati dal lato client senza una firma crittografica. È più facile da configurare poiché non richiede un **API Secret**, ma è meno sicuro perché i dati utente non vengono verificati lato server.

#### Nessuno

Nessuna integrazione SSO. Gli utenti commentano in modo anonimo o devono accedere separatamente a FastComments. Usa questa opzione se non vuoi che gli account Moodle siano collegati ai commenti.

---