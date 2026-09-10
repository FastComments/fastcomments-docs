## Esempi di Zap

Alcuni flussi di lavoro che richiedono pochi minuti per essere configurati.

**Ricevi una notifica per i nuovi commenti.** Nuovo commento, poi Slack "Send Channel Message" o Discord "Send Channel Message". Mappa il nome del commentatore, il testo del commento e l'URL della pagina nel messaggio. Aggiungi il filtro di dominio per notificare un canale diverso per sito.

**Mantieni un registro di ogni commento.** Nuovo commento, poi Google Sheets "Create Spreadsheet Row". Aggiungi Deleted Comment come secondo Zap che aggiunge una riga con l'ID del commento, così il foglio funge anche da tracciamento di audit.

**Invia un'email all'autore quando un commento viene approvato.** Updated Comment con un filtro Zapier su Approved è true, poi Gmail "Send Email". Poiché Updated Comment si attiva a ogni modifica, il filtro è ciò che fa reagire questo Zap solo alle approvazioni.

**Aggiungi i commentatori al tuo CRM o alla tua mailing list.** Nuovo commento, poi HubSpot "Create or Update Contact" o Mailchimp "Add or Update Subscriber" usando l'email del commentatore. Rispetta la tua politica sulla privacy e le leggi locali prima di aggiungere qualcuno a una lista di marketing.

**Crea un commento da un modulo.** Typeform o Google Forms "New Response", poi FastComments Create Comment con l'ID URL della pagina che il tuo sito usa per le testimonianze. Lascia Approved deselezionato per rivedere ciascuno prima che appaia.

**Pubblica annunci su un feed.** RSS by Zapier "New Item in Feed", poi Create Feed Post con il titolo, il contenuto e il link dell'elemento.

**Fornisci i membri come utenti SSO.** Memberstack, Memberful, o il tuo webhook, poi Find SSO User seguito da Create SSO User in modalità "find or create".

**Escalare i commenti segnalati.** Updated Comment, filtrato su un conteggio di segnalazioni superiore a zero, poi Trello "Create Card" o Linear "Create Issue" con l'ID del commento e un link alla pagina di moderazione.

**Pubblica le pagine quando vanno in diretta.** WordPress o Ghost "New Post", poi Create Page con l'URL del post, così la pagina è elencata e limitata prima del primo commento.

**Archivia i commenti eliminati.** Deleted Comment, poi Airtable "Create Record" con il commento completo per la conservazione conforme.