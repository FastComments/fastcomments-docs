Vsak agent webhook je podpisan z HMAC-SHA256 z uporabo API skrivnosti vašega najemnika. Enak način podpisovanja se uporablja za FastCommentsove webhooke za komentarje - če ste jih že integrirali, agent webhooki ponovno uporabijo isti glavi podpisa in tok preverjanja.

### Zakaj podpisovanje

Brez podpisa bi napadalec, ki pozna vaš URL webhooka, lahko poslal ponarejene dogodke, ki izgledajo, kot da prihajajo od FastComments. Podpisovanje pomeni, da vaša končna točka lahko pred ukrepanjem preveri, ali je vsaka dostava avtentična.

### Kako delujejo podpisi

Za vsako dostavo:

1. Platforma poišče API skrivnost za najemnika + ujemajočo se domeno (glejte [Webhooks Overview](#webhooks-overview)).
2. Izda trenutni Unix časovni žig (v milisekundah) v glavi `X-FastComments-Timestamp`.
3. Izračuna `HMAC-SHA256(api_secret, "${timestamp}.${raw_request_body}")` (na način Stripe) in rezultat izda kot `sha256=<hex>` v glavi `X-FastComments-Signature`.
4. Vaša končna točka prebere glavo s časovnim žigom, znova izračuna HMAC čez `${timestamp}.${body}`, ki ga je prejela, primerja z vrednostjo `sha256=<hex>` v glavi podpisa in zavrne neusklajenosti.

Telo, ki je podpisano, so **natanko bajti**, ki jih je platforma poslala, s predpono `${timestamp}.` — vaš preverjalnik mora uporabiti surovo telo zahteve, ne ponovno-serializiran JSON niz (urejanje ključev in presledki bi bili sicer drugačni).

### API skrivnost

Enaka API skrivnost, ki jo uporabljajo [comment webhooks](/guide-webhooks.html). Je za (najemnik, domena) in se upravlja v nastavitvah API vašega najemnika. Če zamenjate skrivnost, bi morali znova namestiti vaš preverjalnik, da prebere novo vrednost pred naslednjo dostavo.

Ko platforma ne najde **no API secret** za ujemajočo se domeno, dostava ne poteka. Dnevnik webhookov zabeleži napako z razlogom "no API secret".

### Primer preverjanja (Node.js)

[inline-code-attrs-start title = 'Primer preverjanja podpisa webhooka'; type='javascript' inline-code-attrs-end]
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

Uporabite `timingSafeEqual` namesto `===`, da se izognete puščanju podpisa preko časovnega kanala.

### Kaj je v podpisanem telesu

Celoten ovojnica in blok `data`, specifičen za dogodek. Glejte [Webhook Payloads](#webhook-payloads).

### Priporočila

- **Preverjajte pri vsaki dostavi.** Če vaša končna točka sprejema nepodpisane zahteve, nimate jamstva o integriteti.
- **Zavrnite ob neusklajenosti podpisa.** Vrnite 401 ali 403; ne vračajte 200 OK ob napačnem podpisu, saj boste sicer prikrili napade v dnevnikih dostave.
- **Uporabljajte HTTPS.** Podpisi ščitijo integriteto; TLS ščiti zaupnost (tako vaše skrivnosti kot besedilo komentarja v payloadu).
- **Zamenjajte skrivnosti** ko člani ekipe s dostopom odidejo ali po urniku.

### Zaščita pred ponovitvami

Samo podpisovanje ne preprečuje napadov ponovnega predvajanja - napadalec, ki je zajel resnično podpisano dostavo, jo lahko ponovno pošlje. Zaščita pred ponovitvami je odgovornost vaše končne točke:

- Uporabite polje ovojnice `occurredAt` in zavrnite dostave, starejše od, recimo, 5 minut.
- Uporabite `triggerId` ali `approvalId` kot ključ za deduplikacijo - če ste ga že obdelali, ignorirajte podvojitev.

### Oglejte si tudi

- [Webhooks Overview](#webhooks-overview).
- [Webhook Payloads](#webhook-payloads).
- Glavni [Webhooks guide](/guide-webhooks.html) za širšo infrastrukturo podpisovanja.