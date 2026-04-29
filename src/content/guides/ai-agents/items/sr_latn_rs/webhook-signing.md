Svaki agent webhook potpisan je pomoću HMAC-SHA256 koristeći API secret vašeg tenanta. Isti način potpisivanja se koristi i za FastComments-ove webhook-ove za komentare - ako ste već integrisali te, agent webhook-ovi ponovo koriste isti signature header i tok verifikacije.

### Zašto potpisivanje

Bez potpisa, napadač koji zna vaš webhook URL može poslati lažne događaje koji izgledaju kao da su poslati od FastComments-a. Potpisivanje znači da vaš endpoint može da verifikuje svaku isporuku kao autentičnu pre nego što preduzme bilo kakvu akciju.

### Kako potpisi funkcionišu

Za svaku isporuku:

1. Platforma traži API secret za tenant + odgovarajući domen (vidi [Webhooks Overview](#webhooks-overview)).
2. Emituje trenutni Unix timestamp (u milisekundama) u `X-FastComments-Timestamp` headeru.
3. Izračunava `HMAC-SHA256(api_secret, "${timestamp}.${raw_request_body}")` (u Stripe stilu) i emituje rezultat kao `sha256=<hex>` u `X-FastComments-Signature` headeru.
4. Vaš endpoint čita timestamp header, ponovo računa HMAC preko `${timestamp}.${body}` koji je primio, upoređuje ga sa `sha256=<hex>` vrednošću u signature headeru i odbacuje neusaglašenosti.

Telo koje se potpisuje su **tačni bajtovi** koje je platforma poslala, prefiksovani sa `${timestamp}.` - vaš verifikator mora da koristi raw request body, a ne ponovo-serializovani JSON string (redosled ključeva i razmaci bi u suprotnom bili različiti).

### API secret

Isti API Secret koji se koristi za [webhook-ove za komentare](/guide-webhooks.html). On je po (tenant, domen) i upravlja se u podešavanjima API-ja vašeg tenanta. Ako promenite secret, treba da ponovo postavite vaš verifikator da pročita novu vrednost pre sledeće isporuke.

Kada platforma ne pronađe **nijedan API secret** za odgovarajući domen, isporuka se ne izvršava. Webhook log beleži neuspeh sa razlogom "no API secret".

### Verification example (Node.js)

[inline-code-attrs-start title = 'Primer verifikacije potpisa webhook-a'; type='javascript' inline-code-attrs-end]
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

Koristite `timingSafeEqual` umesto `===` kako biste izbegli curenja informacija kroz timing-kanal potpisa.

### Šta se nalazi u potpisanom telu

Cela omotnica (envelope) plus event-specifičan `data` blok. Vidi [Webhook Payloads](#webhook-payloads).

### Preporuke

- **Verifikujte pri svakoj isporuci.** Ako vaš endpoint prihvata nepotpisane zahteve, nemate garanciju integriteta.
- **Odbacite pri neusaglašenom potpisu.** Vratite 401 ili 403; ne vraćajte 200 OK na pogrešan potpis, jer ćete na taj način sakriti napade u vašim logovima isporuke.
- **Koristite HTTPS.** Potpisi štite integritet; TLS štiti poverljivost (i vaš secret i tekst komentara u payload-u).
- **Rotirajte secret-e** kada članovi tima sa pristupom odu, ili po rasporedu.

### Zaštita od replay napada

Samo potpisivanje ne sprečava replay napade - napadač koji je zabeležio pravu potpisanu isporuku može je ponovo poslati. Zaštita od replay-a je na strani vašeg endpoint-a:

- Koristite `occurredAt` polje iz omotnice i odbacite isporuke starije od, recimo, 5 minuta.
- Koristite `triggerId` ili `approvalId` kao ključ za deduplikaciju - ako ste ga već obradili, ignorišite duplikat.

### Vidi takođe

- [Webhooks Overview](#webhooks-overview).
- [Webhook Payloads](#webhook-payloads).
- Glavni [Webhooks guide](/guide-webhooks.html) za širu infrastrukturu potpisivanja.