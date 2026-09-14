En val er en naturlig webhook-modtager: den har en stabil URL, den kan verificere en signatur, og den har SQLite og blob-lagring indbygget.

FastComments signerer `${timestamp}.${body}` med din kontos API-hemmelighed og sender to headers:

[inline-code-attrs-start title = 'Webhook-overskrifter'; type='text' inline-code-attrs-end]
[inline-code-start]
X-FastComments-Timestamp: 1789004710        unix seconds, not milliseconds
X-FastComments-Signature: sha256=<hex>
[inline-code-end]

Metoden bærer hændelsen: **PUT** for en oprettet eller opdateret kommentar, **DELETE** for en slettet.

[inline-code-attrs-start title = 'Verificering af en levering'; type='javascript' inline-code-attrs-end]
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

## To ting der bider

**Verificer de rå bytes.** Parsing af JSON og gen-serialisering ændrer nøgleordren og mellemrum, så hash'en er forskellig, og hver levering fejler uden en åbenlys årsag. Dette er den sædvanlige grund til, at en webhook-modtager "bare ikke virker".

**Sammenlign i konstant tid.** En simpel `===` på signaturen lækker hvor mange bytes der matchede, hvilket er nok til at forfalske en byte ad gangen.

## Håndtering af hændelser

Svar hurtigt. FastComments forsøger igen ved et svar, der ikke er 2xx, og et endpoint, der fortsat fejler, deaktiveres til sidst automatisk, så udfør reelt arbejde efter at have svaret i stedet for inline.

Gør arbejdet idempotent på kommentar-id'et. Et retry bliver gen-signeret med en frisk tidsstempel, og det samme kommentar-id ankommer igen ved redigering og sletning, så der er intet stabilt at deduplere på.

---