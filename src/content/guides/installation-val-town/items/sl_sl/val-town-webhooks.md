A val je naravni prejemnik webhookov: ima stabilen URL, lahko preveri podpis in ima vgrajeno SQLite ter shranjevanje blobov.

FastComments podpiše `${timestamp}.${body}` z API skrivnostjo vašega računa in pošlje dva glavi:

[inline-code-attrs-start title = 'Glave webhooka'; type='text' inline-code-attrs-end]
[inline-code-start]
X-FastComments-Timestamp: 1789004710        unix seconds, not milliseconds
X-FastComments-Signature: sha256=<hex>
[inline-code-end]

Metoda prenaša dogodek: **PUT** za ustvarjen ali posodobljen komentar, **DELETE** za izbrisan komentar.

[inline-code-attrs-start title = 'Preverjanje dostave'; type='javascript' inline-code-attrs-end]
[inline-code-start]
import { createHmac, timingSafeEqual } from "node:crypto";

async function receive(c) {
  // Natančni bajti, ki so prispeli. NE uporabljajte c.req.json() in ponovno serijalizirajte.
  const rawBody = await c.req.raw.text();
  const timestamp = c.req.raw.headers.get("X-FastComments-Timestamp");
  const signature = c.req.raw.headers.get("X-FastComments-Signature");

  if (!timestamp || !signature) return new Response("Missing headers", { status: 400 });

  // Zavrnite zastarele dostave, da zajetega zahtevka ne morete kasneje ponovno predvajati.
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

  // ...obdelajte JSON.parse(rawBody)
  return Response.json({ received: true });
}

app.put("/", receive);
app.delete("/", receive);
[inline-code-end]

## Dve stvari, ki povzročata težave

**Preverite surove bajte.** Parsiranje JSON-a in ponovno serijaliziranje spremeni vrstni red ključev in presledke, zato se hash razlikuje in vsaka dostava spodleti brez očitnega vzroka. To je običajen razlog, da prejemnik webhooka "preprosto ne deluje".

**Primerjajte v konstantnem času.** Preprosto `===` na podpisu razkrije, koliko bajtov se ujema, kar je dovolj za podvajanje enega bajta naenkrat.

## Obdelava dogodkov

Odgovorite hitro. FastComments ponavlja poizvedbe pri ne-2xx odgovoru, in končna točka, ki stalno odpoveduje, je na koncu samodejno onemogočena, zato opravite dejansko delo po odgovoru, namesto v isti zahtevi.

Poskrbite, da bo delo idempotentno glede na ID komentarja. Ponovna poizvedba je ponovno podpisana s svežim časovnim žigom, in isti ID komentarja ponovno prispe pri urejanju in brisanju, zato ni nič stabilnega, na čemer bi se lahko deduplikiralo.