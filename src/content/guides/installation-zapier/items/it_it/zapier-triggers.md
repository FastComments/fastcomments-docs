## Trigger

I trigger avviano uno Zap quando si verifica qualcosa in FastComments. Tutti e tre sono istantanei: FastComments invia l'evento a Zapier tramite un webhook nel momento in cui accade. Nessuno interroga il tuo account e non vengono spesi crediti API in attesa.

| Trigger | Si attiva quando |
|---------|-------------------|
| Nuovo commento | Un commento viene pubblicato. Per impostazione predefinita, solo i commenti approvati e non spam attivano il trigger. |
| Commento aggiornato | Un commento viene modificato, approvato, votato, evidenziato, bloccato o altrimenti modificato. |
| Commento eliminato | Un commento viene eliminato. |

Ogni trigger restituisce il commento completo: id, URL della pagina e ID URL, nome e email del commentatore, il testo del commento in markdown e in HTML, conteggi dei voti, flag di approvazione e spam, la lingua, il dominio e eventuali menzioni. I campi corrispondono al payload del webhook documentato nella sezione Webhooks, Strutture dati.

## Opzioni

**Dominio.** Ogni trigger ha un filtro dominio opzionale, che elenca i domini configurati sul tuo account. Lascia vuoto per ricevere eventi da tutti i domini.

**Includi commenti non approvati e spam.** Solo sul trigger Nuovo commento. I commenti trattenuti per moderazione o contrassegnati come spam vengono ignorati per impostazione predefinita. Quando tale commento viene approvato in seguito, il trigger Commento aggiornato si attiva per esso, quindi uno Zap che deve reagire a ogni commento che diventa visibile utilizza Commento aggiornato con un filtro sul campo approvato.

## Come funziona la consegna

Attivare uno Zap crea un abbonamento webhook sul tuo account, visibile nella pagina Webhooks con la sorgente **API**. Disattivare lo Zap lo rimuove. I limiti di Zapier si applicano al numero di eventi che accetta al minuto; FastComments ritenta una consegna che fallisce, con un ritardo crescente, e disabilita un abbonamento che continua a fallire per sei giorni. Un abbonamento disabilitato può essere riattivato dalla pagina Webhooks, oppure basta disattivare e riattivare lo Zap per crearne uno nuovo.

Un account può contenere fino a 50 abbonamenti API. Ogni Zap che utilizza un trigger FastComments ne utilizza uno.