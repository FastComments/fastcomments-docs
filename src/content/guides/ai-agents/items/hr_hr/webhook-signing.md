Svaki agent webhook potpisan je s HMAC-SHA256 koristeći API tajnu vašeg tenanta. Isti shema potpisivanja koristi se za FastComments webhookove komentara - ako ste ih već integrirali, agent webhookovi ponovno koriste isti signature header i postupak verifikacije.

### Zašto potpisivanje

Bez potpisa, napadač koji zna URL vašeg webhooka mogao bi POST-ati krivotvorene događaje koji izgledaju kao da dolaze od FastComments. Potpisivanje znači da vaš endpoint može provjeriti autentičnost svake dostave prije nego što na nju reagira.

### Kako rade potpisi

Za svaku dostavu:

1. Platforma dohvaća API tajnu za tenant + podudarnu domenu (vidi [Pregled webhookova](#webhooks-overview)).
2. Emitira trenutni Unix timestamp (u milisekundama) u headeru `X-FastComments-Timestamp`.
3. Izračunava `HMAC-SHA256(api_secret, "${timestamp}.${raw_request_body}")` (Stripe-stil) i emitira rezultat kao `sha256=<hex>` u headeru `X-FastComments-Signature`.
4. Vaš endpoint čita timestamp header, ponovno izračunava HMAC nad `${timestamp}.${body}` koji je primio, uspoređuje s vrijednošću `sha256=<hex>` u signature headeru i odbija neusklađenosti.

Tijelo koje se potpisuje su **točni bajtovi** koje je platforma poslala, s prefiksom `${timestamp}.` - vaš verifikator mora koristiti sirovo tijelo zahtjeva, a ne ponovno serializirani JSON string (redoslijed ključeva i razmaci inače bi se razlikovali).

### API tajna

Ista API tajna koju koriste [webhookovi komentara](/guide-webhooks.html). Odnosi se na par (tenant, domena) i upravlja se u API postavkama vašeg tenanta. Ako rotirate tajnu, trebali biste ponovno implementirati svoj verifikator da čita novu vrijednost prije sljedeće dostave.

Kada platforma ne pronađe **API tajnu** za podudarnu domenu, dostava se ne događa. Webhook log bilježi grešku s razlogom "no API secret".

### Primjer verifikacije (Node.js)

[inline-code-attrs-start title = 'Primjer verifikacije potpisa webhooka'; type='javascript' inline-code-attrs-end]
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

Koristite `timingSafeEqual` umjesto `===` kako biste izbjegli curenje potpisa kroz timing-kanal.

### Što je u potpisanom tijelu

Cijela omotnica plus blok `data` specifičan za događaj. Vidi [Webhook Payloads](#webhook-payloads).

### Preporuke

- **Provjeravajte svaku dostavu.** Ako vaš endpoint prihvaća nepotpisane zahtjeve, nemate jamstvo integriteta.
- **Odbijte kod neusklađenog potpisa.** Vratite 401 ili 403; ne vraćajte 200 OK na loš potpis, ili ćete prikrivati napade u svojim logovima dostave.
- **Koristite HTTPS.** Potpisi štite integritet; TLS štiti povjerljivost (i vašu tajnu i tekst komentara u payloadu).
- **Rotirajte tajne** kada odu članovi tima s pristupom, ili prema rasporedu.

### Zaštita od replay napada

Samo potpisivanje ne sprječava replay napade - napadač koji je uhvatio stvarnu potpisanu dostavu može je ponovno poslati. Zaštita od replay napada je na vašem endpointu:

- Koristite polje `occurredAt` iz omotnice i odbijte dostave starije od, recimo, 5 minuta.
- Koristite `triggerId` ili `approvalId` kao ključ za deduplikaciju - ako ste ga već obradili, zanemarite duplikat.

### Vidi također

- [Pregled webhookova](#webhooks-overview).
- [Webhook Payloads](#webhook-payloads).
- Glavni [vodič za webhookove](/guide-webhooks.html) za širu infrastrukturu potpisivanja.