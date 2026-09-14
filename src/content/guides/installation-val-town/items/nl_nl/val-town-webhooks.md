A val is een natuurlijke webhook‑ontvanger: het heeft een stabiele URL, het kan een handtekening verifiëren, en het heeft SQLite en blob‑opslag ingebouwd.

FastComments ondertekent `${timestamp}.${body}` met het API‑geheim van uw account en stuurt twee headers:

[inline-code-attrs-start title = 'Webhook-headers'; type='text' inline-code-attrs-end]
[inline-code-start]
X-FastComments-Timestamp: 1789004710        unix seconds, not milliseconds
X-FastComments-Signature: sha256=<hex>
[inline-code-end]

De methode draagt het evenement: **PUT** voor een aangemaakt of bijgewerkt commentaar, **DELETE** voor een verwijderd commentaar.

[inline-code-attrs-start title = 'Levering verifiëren'; type='javascript' inline-code-attrs-end]
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

## Twee dingen die bijten

**Verifieer de ruwe bytes.** Het parseren van de JSON en opnieuw serialiseren verandert de sleutelvolgorde en witruimte, waardoor de hash verschilt en elke levering faalt zonder duidelijke oorzaak. Dit is de gebruikelijke reden waarom een webhook‑ontvanger “gewoon niet werkt”.

**Vergelijk in constante tijd.** Een eenvoudige `===` op de handtekening lekt hoeveel bytes overeenkomen, wat genoeg is om één byte per keer te vervalsen.

## Gebeurtenissen afhandelen

Antwoord snel. FastComments probeert opnieuw bij een non-2xx, en een endpoint die blijft falen wordt uiteindelijk automatisch uitgeschakeld, dus voer het echte werk uit na het beantwoorden in plaats van inline.

Maak dat werk idempotent op basis van de commentaar‑id. Een retry wordt opnieuw ondertekend met een verse timestamp, en dezelfde commentaar‑id komt opnieuw terug bij bewerken en verwijderen, dus er is niets stabiels om te dedupliceren op.

---