Tutte le impostazioni si trovano sotto `Administration > Configuration > Content > FastComments` (`/admin/config/content/fastcomments`).

## Obbligatorio

- **Tenant ID** - Il tuo FastComments Tenant ID. Lo trovi in [Impostazioni > API/SSO](https://fastcomments.com/auth/my-account/api) ([UE](https://eu.fastcomments.com/auth/my-account/api)).
- **API Secret** - Richiesto per SSO Sicuro, la verifica delle webhook e la sincronizzazione delle pagine. Trovabile in [Impostazioni > API/SSO](https://fastcomments.com/auth/my-account/api) ([UE](https://eu.fastcomments.com/auth/my-account/api)).

## Stile dei commenti

Scegli il widget che corrisponde a come vuoi che le persone si esprimano sul tuo sito.

- **Live Comments** - Commenti threadati in tempo reale.
- **Streaming Chat** - Interfaccia chat in tempo reale, adatta per eventi e dirette streaming.
- **Collab Chat** - Annotazione tramite selezione del testo nell'area principale dei contenuti. I visitatori evidenziano il testo e avviano una discussione nel contesto.
- **Collab Chat + Comments** - Sia collab chat che commenti standard sulla stessa pagina.

## Modalità SSO

- **Nessuno** - Nessuno SSO. Gli utenti commentano come ospiti o creano un account FastComments.
- **Semplice** - Passa le informazioni utente di Drupal (nome, email, avatar) a FastComments senza verifica lato server.
- **Sicuro** - Usa HMAC-SHA256 per verificare gli utenti Drupal con FastComments. Consigliato quando hai un API Secret configurato.

Consulta la sezione `Single Sign-On (SSO)` per i dettagli.

## Altre impostazioni

- **CDN URL** - Per impostazione predefinita `https://cdn.fastcomments.com`.
- **Site URL** - Per impostazione predefinita `https://fastcomments.com`.
- **Email notifications** - Invia un'email all'autore del contenuto quando viene pubblicato un nuovo commento sul suo contenuto.

Per la residenza dei dati nell'UE, consulta la sezione `EU Data Residency`.