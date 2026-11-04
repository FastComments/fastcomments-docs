Svaki agent webhook je potpisan sa HMAC-SHA256 koristeći API secret vašeg tenanta. Isti šema potpisivanja se koristi za FastComments webhook-ove za komentare - ako ste već integrisali te, agent webhook-ovi koriste isti signature header i tok verifikacije.

### Zašto potpisivanje

Bez potpisa, napadač koji zna URL vašeg webhook-a mogao je da POSTuje lažne evente koji izgledaju kao da dolaze od FastComments. Potpisivanje znači da vaš endpoint može da proveri da je svaka dostava autentična pre nego što reaguje na nju.

### Kako rade potpisi

Za svaku dostavu:

1. Platforma traži API secret za tenant + upareni domen (vidi [Pregled Webhook-ova](#webhooks-overview)).
2. Emituje trenutni Unix timestamp (u milisekundama) u `X-FastComments-Timestamp` header-u.
3. Računa `HMAC-SHA256(api_secret, "${timestamp}.${raw_request_body}")` (Stripe-stil) i emituje rezultat kao `sha256=<hex>` u `X-FastComments-Signature` header-u.
4. Vaš endpoint čita timestamp header, ponovo računa HMAC preko `${timestamp}.${body}` koji je primio, poredi sa `sha256=<hex>` vrednošću u signature header-u i odbija neusaglašenosti.

Telo koje je potpisano su **tačni bajtovi** koje je platforma poslala, sa prefiksom `${timestamp}.` - vaš verifikator mora da koristi sirovo telo zahteva, a ne ponovo serijalizovani JSON string (redosled ključeva i razmaci bi inače bili drugačiji).

### API secret

Isti API Secret koji koriste [webhook-ovi za komentare](/guide-webhooks.html). Odnosi se na (tenant, domen) par i upravlja se u API podešavanjima vašeg tenanta. Ako rotirate secret, trebalo bi da ponovo primenite svoj verifikator da bi čitao novu vrednost pre sledeće dostave.

Kada platforma ne pronađe **API secret** za upareni domen, dostava se ne dešava. Webhook log beleži neuspeh sa razlogom "no API secret".

### Primer verifikacije (Node.js)

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

Koristite `timingSafeEqual` umesto `===` da biste izbegli curenje potpisa kroz timing kanal.

### Šta se nalazi u potpisanom telu

Kompletan envelope plus event-specifični `data` blok. Vidite [Webhook Payloads](#webhook-payloads).

### Preporuke

- **Verifikujte pri svakoj dostavi.** Ako vaš endpoint prihvata nepotpisane zahteve, nemate garanciju integriteta.
- **Odbijajte pri neusaglašenosti potpisa.** Vratite 401 ili 403; ne vraćajte 200 OK na loš potpis, ili ćete prikrivati napade u svojim logovima dostava.
- **Koristite HTTPS.** Potpisi štite integritet; TLS štiti poverljivost (i vaš secret i tekst komentara u payload-u).
- **Rotirajte secret-e** kada članovi tima sa pristupom odu, ili po rasporedu.

### Zaštita od replay napada

Samo potpisivanje ne sprečava replay napade - napadač koji je uhvatio stvarnu potpisanu dostavu može da je ponovo pošalje. Zaštita od replay napada je na vašem endpoint-u:

- Koristite `occurredAt` polje envelope-a i odbijajte dostave starije od, recimo, 5 minuta.
- Koristite `triggerId` ili `approvalId` kao dedup ključ - ako ste ga već obradili, ignorišite duplikat.

### Vidite takođe

- [Pregled Webhook-ova](#webhooks-overview).
- [Webhook Payloads](#webhook-payloads).
- Glavni [vodič za Webhooks](/guide-webhooks.html) za širu infrastrukturu potpisivanja.