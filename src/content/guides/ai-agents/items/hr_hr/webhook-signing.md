Svaki agent webhook potpisan je pomoću HMAC-SHA256 koristeći API secret vašeg tenant-a. Isti način potpisivanja koristi se i za FastCommentsove comment webhooks - ako ste već integrirali te, agent webhookovi koriste isti signature header i tok verifikacije.

### Zašto potpisivanje

Bez potpisa, napadač koji zna vaš webhook URL mogao bi POST-ati falsificirane događaje koji izgledaju kao da dolaze iz FastCommentsa. Potpisivanje znači da vaš endpoint može provjeriti je li svaka isporuka autentična prije nego što poduzme radnju.

### Kako potpisi funkcioniraju

Za svaku isporuku:

1. Platforma dohvaća API secret za tenant + podudarajući se domen (vidi [Pregled Webhookova](#webhooks-overview)).
2. Emitira trenutni Unix timestamp (u milisekundama) u headeru `X-FastComments-Timestamp`.
3. Izračunava `HMAC-SHA256(api_secret, "${timestamp}.${raw_request_body}")` (kao u Stripeu) i rezultat postavlja kao `sha256=<hex>` u header `X-FastComments-Signature`.
4. Vaš endpoint čita timestamp header, ponovno izračunava HMAC preko `${timestamp}.${body}` koji je primio, uspoređuje s vrijednošću `sha256=<hex>` u signature headeru i odbacuje nesukladnosti.

Tijelo koje se potpisuje su **točni bajtovi** koje je platforma poslala, prefiksirani s `${timestamp}.` - vaš verifier mora koristiti raw request body, a ne ponovno serijalizirani JSON string (redoslijed ključeva i razmaci bi inače bili različiti).

### API secret

Isti API Secret koji koristi [comment webhooks](/guide-webhooks.html). On je po (tenant, domain) i upravlja se u postavkama API-ja vašeg tenant-a. Ako promijenite secret, trebate ponovno rasporediti svoj verifier da očita novu vrijednost prije sljedeće isporuke.

Kada platforma ne pronađe **niti jedan API secret** za podudarajuću domenu, isporuka se ne obavlja. Webhook log bilježi neuspjeh s razlogom "no API secret".

### Verification example (Node.js)

[inline-code-attrs-start title = 'Primjer provjere potpisa webhooka'; type='javascript' inline-code-attrs-end]
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

Koristite `timingSafeEqual` umjesto `===` kako biste izbjegli curenje informacije putem timing-kanala o potpisu.

### Što se nalazi u potpisanom tijelu

Cijeli omot (envelope) plus event-specifični `data` blok. Vidi [Webhook Payloads](#webhook-payloads).

### Preporuke

- **Provjeravajte pri svakoj isporuci.** Ako vaš endpoint prihvaća nepotpisane zahtjeve, nemate jamstvo integriteta.
- **Odbacujte pri neslaganju potpisa.** Vratite 401 ili 403; nemojte vraćati 200 OK na loš potpis, jer ćete time prikriti napade u vašim delivery logovima.
- **Koristite HTTPS.** Potpisi štite integritet; TLS štiti povjerljivost (i vašeg secreta i teksta komentara u payloadu).
- **Rotirajte tajne** kada članovi tima s pristupom odu, ili po rasporedu.

### Zaštita od ponovnog slanja

Samo potpisivanje ne sprječava replay napade - napadač koji je presreo stvarnu potpisanu isporuku može je ponovno poslati. Zaštita od replay napada ovisi o vašem endpointu:

- Koristite polje `occurredAt` u envelopeu i odbacite isporuke starije od, recimo, 5 minuta.
- Koristite `triggerId` ili `approvalId` kao ključ za deduplikaciju - ako ste ga već obradili, ignorirajte duplikat.

### Vidi također

- [Pregled Webhookova](#webhooks-overview).
- [Webhook Payloads](#webhook-payloads).
- Glavni [Vodič za Webhookove](/guide-webhooks.html) za širu infrastrukturu potpisivanja.