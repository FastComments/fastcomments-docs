## Azioni e Ricerche

Le azioni creano dati in FastComments; le ricerche cercano dati in modo che un passaggio successivo possa usarli. Ogni azione chiama l'API REST di FastComments e consuma gli stessi crediti API che la chiamata costerebbe dal tuo codice: un credito per chiamata a meno che non sia indicato diversamente.

## Crea Commento

Pubblica un commento su una pagina.

| Campo | Obbligatorio | Note |
|-------|--------------|------|
| ID URL della Pagina | Sì | L'ID URL che il widget dei commenti utilizza sulla pagina. I commenti sono raggruppati per esso. |
| URL della Pagina | Sì | L'URL completo della pagina, usato nelle email di notifica. |
| Commento | Sì | Il corpo del commento in markdown di FastComments. |
| Nome del Commentatore | Sì | I nomi sono unici per email, quindi riutilizzare un nome con un'email diversa fallisce. |
| Email del Commentatore | No | Viene creato un utente per l'email se non esiste ancora. |
| ID Utente | No | Un ID utente SSO esistente. Ha la precedenza su nome ed email. |
| ID Commento Padre | No | Impostare per pubblicare una risposta. |
| Approvato, Verificato | No | Entrambi sono true di default. I commenti non approvati rimangono nascosti fino a quando non vengono moderati. |
| Data di Pubblicazione | No | Il valore predefinito è ora. |
| URL Avatar, Titolo Pagina, Locale | No | Il locale predefinito è `en_us`. |
| Mostra in Tempo Reale nel Widget | No | Invia il commento agli spettatori in tempo reale. Costa 2 crediti invece di 1. |
| Esegui Controllo Spam, Invia Email | No | Disattivati di default. |

## Crea Pagina

Crea un record di pagina prima che esista un commento su di essa, così può essere elencata e limitata. Richiede l'ID URL, il titolo, l'URL e, facoltativamente, gli ID dei gruppi SSO autorizzati a vederla.

## Crea Utente SSO

Crea un utente single sign-on. Richiede il tuo ID utente, nome utente e email, più opzionalmente nome visualizzato, etichetta visualizzata, avatar, sito web, ID gruppi, e flag di notifica e privacy. I ruoli amministrativi non possono essere concessi da Zapier.

## Crea Post Feed

Crea un post in un feed FastComments dal contenuto HTML, con un titolo opzionale, autore, tag e un'anteprima del link.

## Crea Tag Hash

Crea un tag hash che i commentatori possono usare, con un URL opzionale a cui collega.

## Segnala Commento

Segnala un commento per la revisione del moderatore. Fornisci l'ID dell'utente che effettua la segnalazione, o lascialo vuoto per segnalare come integrazione Zapier.

## Ricerche

| Ricerca | Input | Restituisce |
|---------|-------|-------------|
| Trova Commento | ID Commento | Il commento, o nulla. |
| Trova Utente SSO | Email | L'utente SSO, o nulla. |
| Trova Pagina | ID URL | La pagina, o nulla. |

Una ricerca che non trova nulla non fa fallire lo Zap. Combina una ricerca con una creazione nella modalità "trova o crea" di Zapier per creare la pagina o l'utente quando manca.