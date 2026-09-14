A val je prirodni primatelj webhooka: ima stabilan URL, može provjeriti potpis i ima ugrađenu SQLite i blob pohranu.

FastComments potpisuje `${timestamp}.${body}` tajnom API-ja vašeg računa i šalje dva zaglavlja:

[inline-code-attrs-start title = 'Zaglavlja webhooka'; type='text' inline-code-attrs-end]
[inline-code-start]
X-FastComments-Timestamp: 1789004710        unix seconds, not milliseconds
X-FastComments-Signature: sha256=<hex>
[inline-code-end]

Metoda nosi događaj: **PUT** za kreirani ili ažurirani komentar, **DELETE** za izbrisani.

[inline-code-attrs-start title = 'Provjera isporuke'; type='javascript' inline-code-attrs-end]
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

## Dvije stvari koje grizu

**Provjerite sirove bajtove.** Parsiranje JSON-a i ponovno serijaliziranje mijenja redoslijed ključeva i razmake, pa se hash razlikuje i svaka isporuka ne uspijeva bez očiglednog uzroka. Ovo je uobičajeni razlog da primatelj webhooka „just doesn't work“.

**Usporedite u konstantnom vremenu.** Obični `===` na potpis otkriva koliko bajtova se podudara, što je dovoljno za falsificiranje jednog bajta po jedan.

## Obrada događaja

Odgovorite brzo. FastComments ponavlja pokušaje kod ne‑2xx odgovora, a krajnja točka koja stalno ne uspijeva na kraju se automatski onemogućuje, stoga stvarni rad obavite nakon odgovora, a ne inline.

Učinite da rad bude idempotentan po ID‑u komentara. Ponovni pokušaj se ponovno potpisuje s novim vremenskim žigom, a isti ID komentara ponovno dolazi pri uređivanju i brisanju, pa nema ničeg stabilnog na čemu se može deduplicirati.