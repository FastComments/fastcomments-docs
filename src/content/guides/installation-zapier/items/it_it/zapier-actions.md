## Azioni e Ricerche

Le azioni creano dati in FastComments; le ricerche cercano dati in modo che un passaggio successivo possa usarli. Ogni azione chiama l'API REST di FastComments e consuma gli stessi crediti API che la chiamata costerebbe dal tuo codice: un credito per chiamata, salvo indicazione contraria.

## Crea Commento

Pubblica un commento su una pagina.

| Field | Required | Notes |
|-------|----------|-------|
| Page URL ID | Yes | L'ID URL che il widget dei commenti utilizza nella pagina. I commenti sono raggruppati in base a esso. |
| Page URL | Yes | L'URL completo della pagina, usato nelle email di notifica. |
| Comment | Yes | Il corpo del commento in markdown di FastComments. |
| Commenter Name | Yes | I nomi sono unici per email, quindi riutilizzare un nome con un'email diversa fallisce. |
| Commenter Email | No | Viene creato un utente per l'email se non esiste ancora. |
| User ID | No | Un ID utente SSO esistente. Ha la precedenza su nome ed email. |
| Parent Comment ID | No | Impostare per pubblicare una risposta. |
| Approved, Verified | No | Entrambi hanno valore predefinito true. I commenti non approvati rimangono nascosti fino a quando non vengono moderati. |
| Posted At | No | Il valore predefinito è l'ora corrente. |
| Avatar URL, Page Title, Locale | No | Il locale predefinito è `en_us`. |
| Show Live In Widget | No | Invia il commento agli spettatori in tempo reale. Costa 2 crediti invece di 1. |
| Run Spam Check, Send Emails | No | Disattivato per impostazione predefinita. |

## Crea Pagina

Crea un record di pagina prima che vi sia un commento, così può essere elencata e limitata. Richiede l'ID URL, il titolo, l'URL e, facoltativamente, gli ID dei gruppi SSO autorizzati a vederla.

## Crea Utente SSO

Crea un utente single sign-on. Richiede il tuo ID utente, nome utente e email, più opzionalmente nome visualizzato, etichetta visualizzata, avatar, sito web, ID dei gruppi e flag di notifica e privacy. I ruoli amministrativi non possono essere concessi da Zapier.

## Crea Post Feed

Crea un post in un feed FastComments a partire da contenuto HTML. L'ID utente dell'autore è obbligatorio (un ID utente FastComments o SSO); titolo, tag e un'anteprima del link sono opzionali.

## Crea Hashtag

Crea un hashtag che i commentatori possono usare, con un URL opzionale a cui collega. Gli hashtag sono unici per account, quindi uno Zap che ne crea uno ad ogni esecuzione ha bisogno di qualcosa di unico nell'hashtag.

## Segnala Commento

Segnala un commento per la revisione del moderatore. È richiesto l'ID dell'utente che effettua la segnalazione; funziona l'ID autore restituito da Crea Commento.

## Ricerche

| Search | Input | Returns |
|--------|-------|---------|
| Find Comment | Comment ID | Il commento, o nulla. |
| Find SSO User | Email | L'utente SSO, o nulla. |
| Find Page | URL ID | La pagina, o nulla. |

Una ricerca che non trova nulla non fa fallire lo Zap. Combina una ricerca con una creazione nella modalità "find or create" di Zapier per creare la pagina o l'utente quando manca.