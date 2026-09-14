A val è un ricevitore webhook naturale: ha un URL stabile, può verificare una firma e include SQLite e storage blob integrati.

FastComments firma `${timestamp}.${body}` con il segreto API del tuo account e invia due intestazioni:

[inline-code-attrs-start title = 'Intestazioni webhook'; type='text' inline-code-attrs-end]
[inline-code-start]
X-FastComments-Timestamp: 1789004710        unix seconds, not milliseconds
X-FastComments-Signature: sha256=<hex>
[inline-code-end]

Il metodo trasporta l'evento: **PUT** per un commento creato o aggiornato, **DELETE** per uno eliminato.

[inline-code-attrs-start title = 'Verifica di una consegna'; type='javascript' inline-code-attrs-end]
[inline-code-start]
import { createHmac, timingSafeEqual } from "node:crypto";

async function receive(c) {
  // The exact bytes that arrived. Do NOT use c.req.json() and re-serialize.
  const rawBody = await c.req.raw.text();
  const timestamp = c.req.raw.headers.get("X-FastComments-Timestamp");
  const signature = c.req.raw.headers.get("X-FastComments-Signature");

  if (!timestamp || !signature) return new Response("Missing headers", { status: 400 });

  // Reject stale deliveries so a captured request cannot be replayed later.
  if (Math.abs(Math.floor(Date.now() / 1000) - Number(timestamp)) > 300) {
    return new Response("Timestamp outside window", { status: 400 });
  }

  const expected = "sha256=" + createHmac("sha256", Deno.env.get("FASTCOMMENTS_API_SECRET"))
    .update(`${timestamp}.${rawBody}`)
    .digest("hex");

  const a = new TextEncoder().encode(signature);
  const b = new TextEncoder().encode(expected);
  if (a.length !== b.length || !timingSafeEqual(a, b)) {
    return new Response("Signature mismatch", { status: 401 });
  }

  // ...handle JSON.parse(rawBody)
  return Response.json({ received: true });
}

app.put("/", receive);
app.delete("/", receive);
[inline-code-end]

## Due cose che mordono

**Verifica i byte grezzi.** L'analisi del JSON e la sua riserializzazione cambiano l'ordine delle chiavi e gli spazi, quindi l'hash differisce e ogni consegna fallisce senza una causa evidente. Questo è il motivo più comune per cui un ricevitore webhook "semplicemente non funziona".

**Confronta in tempo costante.** Un semplice `===` sulla firma rivela quanti byte corrispondono, il che è sufficiente per forgiarne uno alla volta.

## Gestione degli eventi

Rispondi rapidamente. FastComments riprova in caso di risposta non 2xx, e un endpoint che continua a fallire viene disabilitato automaticamente, quindi esegui il lavoro reale dopo aver risposto anziché inline.

Rendi quel lavoro idempotente sull'ID del commento. Un retry viene firmato nuovamente con un timestamp nuovo, e lo stesso ID del commento arriva di nuovo in caso di modifica o eliminazione, quindi non c'è nulla di stabile su cui deduplicare.

---