All settings live under `Administration > Configuration > Content > FastComments` (`/admin/config/content/fastcomments`).

## Obbligatorio

- **Tenant ID** – Il tuo FastComments Tenant ID. Trovalo in [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).
- **API Secret** – Necessario per Secure SSO, verifica webhook e sincronizzazione della pagina. Si trova in [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).

## Memorizzazione del segreto API

Il modulo legge il segreto API dalla prima di queste fonti che contiene un valore:

1. **Key module** – Installa [Key](https://www.drupal.org/project/key), crea una chiave *Authentication* e selezionala in **API Secret key** nel modulo delle impostazioni. La chiave può essere basata su una variabile d'ambiente o su un file al di fuori della radice web.
2. **settings.php** – Aggiungi `$config['fastcomments.settings']['api_secret'] = 'your-secret';` a `settings.php` o `settings.local.php`. Il segreto non entra mai nella configurazione esportata.
3. **Il modulo delle impostazioni** – Inseriscilo nel campo **API Secret**. Questo memorizza il valore in `fastcomments.settings`, quindi finisce nelle esportazioni di configurazione. Il rapporto di stato di Drupal avvisa quando questa è la fonte attiva.

Il modulo mostra quale fonte è attiva. Lasciare il campo vuoto mantiene il segreto memorizzato. Seleziona **Remove the API Secret stored in configuration** per eliminarlo dopo aver spostato il segreto nel modulo Key o in `settings.php`.

## Stile di commento

Scegli il widget che corrisponde a come vuoi che le persone parlino sul tuo sito.

- **Live Comments** – Commenti in tempo reale con struttura ad albero.
- **Streaming Chat** – Interfaccia di chat in diretta, buona per eventi e livestream.
- **Collab Chat** – Annotazione tramite selezione di testo nell'area principale del contenuto. I visitatori evidenziano il testo e avviano una discussione nel contesto.
- **Collab Chat + Comments** – Sia chat collaborativa che commenti standard nella stessa pagina.

## Modalità SSO

- **None** – Nessun SSO. Gli utenti commentano come ospiti o creano un account FastComments.
- **Simple** – Passa le informazioni utente di Drupal (nome, email, avatar) a FastComments senza verifica lato server.
- **Secure** – Utilizza HMAC‑SHA256 per verificare gli utenti Drupal con FastComments. Consigliato quando è configurato un segreto API.

Vedi la sezione `Single Sign-On (SSO)` per i dettagli.

## Altre impostazioni

- **CDN URL** – Predefinito a `https://cdn.fastcomments.com`.
- **Site URL** – Predefinito a `https://fastcomments.com`.
- **Email notifications** – Invia un'email all'autore del contenuto quando viene pubblicato un nuovo commento sul suo contenuto.

Per la residenza dei dati UE, vedi la sezione `EU Data Residency`.