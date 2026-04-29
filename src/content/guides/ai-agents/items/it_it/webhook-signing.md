---
Ogni agent webhook è firmato con HMAC-SHA256 usando il secret API del tuo tenant. Lo stesso schema di firma è usato per i webhook dei commenti di FastComments - se li hai già integrati, i webhook agent riutilizzano la stessa intestazione di firma e il medesimo flusso di verifica.

### Perché firmare

Senza una firma, un attaccante che conosce l'URL del tuo webhook potrebbe inviare con POST eventi contraffatti che sembrano provenire da FastComments. La firma consente al tuo endpoint di verificare che ogni consegna sia autentica prima di agire.

### Come funzionano le firme

Per ogni consegna:

1. La piattaforma ricerca il secret API per il tenant + dominio corrispondente (vedi [Webhooks Overview](#webhooks-overview)).
2. Emette il timestamp Unix corrente (in millisecondi) nell'intestazione `X-FastComments-Timestamp`.
3. Calcola `HMAC-SHA256(api_secret, "${timestamp}.${raw_request_body}")` (in stile Stripe) ed emette il risultato come `sha256=<hex>` nell'intestazione `X-FastComments-Signature`.
4. Il tuo endpoint legge l'intestazione del timestamp, ricalcola l'HMAC su `${timestamp}.${body}` ricevuto, lo confronta con il valore `sha256=<hex>` nell'intestazione di firma, e rifiuta le discrepanze.

Il corpo che viene firmato è i **byte esatti** inviati dalla piattaforma, prefissati da `${timestamp}.` - il tuo verificatore deve usare il corpo della richiesta raw, non una stringa JSON ri-serializzata (l'ordinamento delle chiavi e gli spazi bianchi risulterebbero altrimenti differenti).

### API secret

Lo stesso API Secret usato dai [comment webhooks](/guide-webhooks.html). È per (tenant, domain) e gestito nelle impostazioni API del tuo tenant. Se ruoti il secret, dovresti ridistribuire il tuo verificatore per leggere il nuovo valore prima della prossima consegna.

Quando la piattaforma trova **no API secret** per il dominio corrispondente, la consegna non avviene. Il log dei webhook registra il fallimento con motivo "no API secret".

### Verification example (Node.js)

[inline-code-attrs-start title = 'Esempio di verifica della firma del webhook'; type='javascript' inline-code-attrs-end]
[inline-code-start]
import crypto from 'crypto';

function verifyAgentWebhook(rawBody, signatureHeader, timestampHeader, secret) {
  const expected = 'sha256=' + crypto
    .createHmac('sha256', secret)
    .update(`${timestampHeader}.${rawBody}`)
    .digest('hex');
  return crypto.timingSafeEqual(
    Buffer.from(expected),
    Buffer.from(signatureHeader),
  );
}
[inline-code-end]

Usa `timingSafeEqual` invece di `===` per evitare perdite attraverso canali di temporizzazione della firma.

### Cosa contiene il corpo firmato

L'intera envelope più il blocco `data` specifico dell'evento. Vedi [Webhook Payloads](#webhook-payloads).

### Raccomandazioni

- **Verifica ad ogni consegna.** Se il tuo endpoint accetta richieste non firmate, non hai garanzia di integrità.
- **Rifiuta in caso di mismatch della firma.** Restituisci 401 o 403; non restituire 200 OK a una firma invalida, altrimenti maschererai gli attacchi nei log delle consegne.
- **Usa HTTPS.** Le firme proteggono l'integrità; TLS protegge la riservatezza (sia del tuo secret che del testo del commento nel payload).
- **Ruota i secret** quando membri del team con accesso se ne vanno, o secondo una pianificazione.

### Protezione contro i replay

La sola firma non previene gli attacchi di replay - un attaccante che ha catturato una consegna firmata reale può reinviarla. La protezione contro i replay spetta al tuo endpoint:

- Usa il campo `occurredAt` dell'envelope e rifiuta le consegne più vecchie, ad esempio, di oltre 5 minuti.
- Usa `triggerId` o `approvalId` come chiave di deduplicazione - se l'hai già elaborata, ignora il duplicato.

### Vedi anche

- [Webhooks Overview](#webhooks-overview).
- [Webhook Payloads](#webhook-payloads).
- La guida principale sui [Webhooks](/guide-webhooks.html) per l'infrastruttura di firma più ampia.

---