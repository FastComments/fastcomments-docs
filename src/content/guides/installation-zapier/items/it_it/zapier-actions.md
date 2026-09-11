## Azioni e Ricerche

Le azioni creano dati in FastComments; le ricerche recuperano dati in modo che un passaggio successivo possa usarli. Ogni azione chiama l'API REST di FastComments e consuma gli stessi crediti API che la chiamata costerebbe dal tuo codice: un credito per chiamata, salvo diversa indicazione.

## Crea Commento

Pubblica un commento su una pagina.

| Campo | Obbligatorio | Note |
|-------|--------------|------|
| Page URL ID | Sì | L'ID URL che il widget dei commenti utilizza sulla pagina. I commenti sono raggruppati per questo. |
| Page URL | Sì | L'URL completo della pagina, usato nelle email di notifica. |
| Comment | Sì | Il corpo del commento in markdown di FastComments. |
| Commenter Name | Sì | I nomi sono unici per email, quindi riutilizzare un nome con un'email diversa fallisce. |
| Commenter Email | No | Un utente viene creato per l'email se non esiste ancora. |
| User ID | No | Un ID utente SSO esistente. Ha la precedenza su nome ed email. |
| Parent Comment ID | No | Impostare per pubblicare una risposta. |
| Approved, Verified | No | Entrambi predefiniti a true. I commenti non approvati rimangono nascosti fino a quando non vengono moderati. |
| Posted At | No | Predefinito a ora corrente. |
| Avatar URL, Page Title, Locale | No | Il locale predefinito è `en_us`. |
| Show Live In Widget | No | Invia il commento ai visualizzatori in tempo reale. Costa 2 crediti invece di 1. |
| Run Spam Check, Send Emails | No | Disattivati per impostazione predefinita. |

## Crea o Aggiorna Pagina

Crea un record di pagina prima che esista un commento su di essa, così può essere elencata e limitata. Richiede l'ID URL, il titolo, l'URL e, facoltativamente, gli ID dei gruppi SSO autorizzati a vederla. Se una pagina con quell'ID URL esiste già, viene aggiornata con i campi forniti, così uno Zap può essere eseguito più volte per la stessa pagina.

## Crea o Aggiorna Utente SSO

Crea un utente single sign‑on. Richiede il tuo ID utente, nome utente ed email, più opzionalmente nome visualizzato, etichetta visualizzata, avatar, sito web, ID dei gruppi e flag di notifica e privacy. Se un utente con quell'ID esiste già, viene aggiornato invece. I ruoli amministrativi non possono essere concessi da Zapier.

## Crea Post del Feed

Crea un post in un feed FastComments a partire da contenuto HTML. È richiesto l'ID utente dell'autore (un ID FastComments o SSO); titolo, tag e un'anteprima del link sono opzionali.

## Crea o Aggiorna Hashtag

Crea un hashtag che i commentatori possono utilizzare, con un URL opzionale a cui collegarlo. Se l'hashtag esiste già, viene aggiornato invece.

## Segnala Commento

Segnala un commento per la revisione di un moderatore. È richiesto l'ID dell'utente che effettua la segnalazione; l'ID autore restituito da Crea Commento funziona.

## Ricerche

| Ricerca | Input | Restituisce |
|--------|-------|-------------|
| Find Comment | Comment ID | Il commento, o nulla. |
| Find SSO User | Email | L'utente SSO, o nulla. |
| Find Page | URL ID | La pagina, o nulla. |

Una ricerca che non trova nulla non fa fallire lo Zap. Find SSO User e Find Page offrono l'opzione "crea se non esiste" di Zapier, che esegue la creazione corrispondente quando non viene trovato alcun risultato.