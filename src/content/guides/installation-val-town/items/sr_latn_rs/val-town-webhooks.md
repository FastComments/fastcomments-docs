A val je prirodni webhook prijemnik: ima stabilan URL, može verifikovati potpis i ima ugrađenu SQLite i blob skladište.

FastComments potpisuje `${timestamp}.${body}` tajnim ključem API-ja vašeg naloga i šalje dva zaglavlja:

[inline-code-attrs-start title = 'Zaglavlja webhook-a'; type='text' inline-code-attrs-end]
[inline-code-start]
X-FastComments-Timestamp: 1789004710        unix seconds, not milliseconds
X-FastComments-Signature: sha256=<hex>
[inline-code-end]

Metoda nosi događaj: **PUT** za kreiran ili ažuriran komentar, **DELETE** za obrisan komentar.

[inline-code-attrs-start title = 'Verifikacija isporuke'; type='javascript' inline-code-attrs-end]
[inline-code-start]
import { createHmac, timingSafeEqual } from "node:crypto";

async function receive(c) {
  // Tačni bajtovi koji su stigli. Nemojte koristiti c.req.json() i ponovo serijalizovati.
  const rawBody = await c.req.raw.text();
  const timestamp = c.req.raw.headers.get("X-FastComments-Timestamp");
  const signature = c.req.raw.headers.get("X-FastComments-Signature");

  if (!timestamp || !signature) return new Response("Missing headers", { status: 400 });

  // Odbaci zastarele isporuke kako uhvaćeni zahtev ne bi mogao biti ponovo reprodukovan kasnije.
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

  // ...obradi JSON.parse(rawBody)
  return Response.json({ received: true });
}

app.put("/", receive);
app.delete("/", receive);
[inline-code-end]

## Dve stvari koje mogu da zagrizu

**Verifikujte sirove bajtove.** Parsiranje JSON-a i njegovo ponovo serijalizovanje menja redosled ključeva i razmake, pa se hash razlikuje i svaka isporuka ne uspeva bez očiglednog uzroka. Ovo je uobičajen razlog da webhook prijemnik „jednostavno ne radi“.

**Uporedite u konstantnom vremenu.** Obična `===` operacija na potpisu otkriva koliko bajtova se podudara, što je dovoljno da se falsifikuje po jedan bajt odjednom.

## Obrada događaja

Odgovorite brzo. FastComments ponovo pokušava na ne‑2xx odgovoru, a krajnja tačka koja stalno ne uspeva se na kraju automatski onemogućava, zato obavite stvarni rad nakon odgovora, a ne inline.

Učinite da rad bude idempotentan po ID‑u komentara. Ponovni pokušaj se ponovo potpisuje svežim timestamp‑om, a isti ID komentara dolazi ponovo pri izmeni i brisanju, pa nema stabilnog podatka za deduplikaciju.