Ogni webhook dell'agente è firmato con HMAC-SHA256 utilizzando il segreto API del tuo tenant. Lo stesso schema di firma è usato per i webhook dei commenti di FastComments - se li hai già integrati, i webhook dell'agente riutilizzano lo stesso header di firma e lo stesso flusso di verifica.

### Perché firmare

Senza una firma, un attaccante che conosce l'URL del tuo webhook potrebbe inviare eventi contraffatti che sembrano provenire da FastComments. La firma consente al tuo endpoint di verificare che ogni consegna sia autentica prima di agire di conseguenza.

### Come funzionano le firme

Per ogni consegna:

1. La piattaforma cerca il segreto API per il tenant + dominio corrispondente (vedi [Panoramica dei Webhook](#webhooks-overview)).
2. Invia il timestamp Unix corrente (in millisecondi) nell'header `X-FastComments-Timestamp`.
3. Calcola `HMAC-SHA256(api_secret, "${timestamp}.${raw_request_body}")` (in stile Stripe) e invia il risultato come `sha256=<hex>` nell'header `X-FastComments-Signature`.
4. Il tuo endpoint legge l'header del timestamp, ricalcola l'HMAC su `${timestamp}.${body}` ricevuto, lo confronta con il valore `sha256=<hex>` nell'header di firma e rifiuta le corrispondenze errate.

Il corpo firmato sono gli **esatti byte** inviati dalla piattaforma, preceduti da `${timestamp}.` - il tuo verificatore deve usare il corpo della richiesta raw, non una stringa JSON ri-serializzata (altrimenti l'ordine delle chiavi e gli spazi bianchi differirebbero).

### Segreto API

È lo stesso Segreto API usato dai [webhook dei commenti](/guide-webhooks.html). È per (tenant, dominio) e gestito nelle impostazioni API del tuo tenant. Se ruoti il segreto, dovresti ridistribuire il tuo verificatore per leggere il nuovo valore prima della prossima consegna.

Quando la piattaforma non trova **nessun segreto API** per il dominio corrispondente, la consegna non avviene. Il log del webhook registra l'errore con motivo "no API secret".

### Esempio di verifica (Node.js)

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

Usa `timingSafeEqual` invece di `===` per evitare perdite della firma tramite canali temporali (timing-channel).

### Cosa contiene il corpo firmato

L'intera busta più il blocco `data` specifico dell'evento. Vedi [Payload dei Webhook](#webhook-payloads).

### Raccomandazioni

- **Verifica ad ogni consegna.** Se il tuo endpoint accetta richieste non firmate, non hai alcuna garanzia di integrità.
- **Rifiuta in caso di firma non corrispondente.** Restituisci 401 o 403; non restituire 200 OK con una firma errata, altrimenti maschererai gli attacchi nei tuoi log di consegna.
- **Usa HTTPS.** Le firme proteggono l'integrità; TLS protegge la riservatezza (sia il tuo segreto sia il testo dei commenti nel payload).
- **Ruota i segreti** quando membri del team con accesso se ne vanno, o secondo una pianificazione regolare.

### Protezione dal replay

La firma da sola non previene gli attacchi di replay - un attaccante che ha catturato una consegna firmata reale può inviarla nuovamente. La protezione dal replay è responsabilità del tuo endpoint:

- Usa il campo della busta `occurredAt` e rifiuta le consegne più vecchie di, ad esempio, 5 minuti.
- Usa `triggerId` o `approvalId` come chiave di deduplicazione - se l'hai già elaborato, ignora il duplicato.

### Vedi anche

- [Panoramica dei Webhook](#webhooks-overview).
- [Payload dei Webhook](#webhook-payloads).
- La [guida principale ai Webhook](/guide-webhooks.html) per l'infrastruttura di firma più ampia.