Strani za nov in urejanje webhook imata gumb `Send Test Payload`, ki pošlje zahtevo na URL, ki je trenutno v obrazcu, ne glede na to, ali je bil shranjen ali ne. Dogodka Create in Update pošljeta lažni objekt WebhookComment, medtem ko testiranje Delete pošlje lažno telo zahteve samo z ID-jem.

## Preverjanje Payloadov

Pri testiranju vaše webhook integracije preverite, ali dohodne zahteve vsebujejo naslednje glave:

1. **`X-FastComments-Timestamp`** - Unix časovni žig (sekunde)
2. **`X-FastComments-Signature`** - HMAC-SHA256 podpis

Webhooki, ustvarjeni pred uvedbo sheme podpisov, prejmejo tudi glavo **`token`**, ki vsebuje vaš API Secret. Novi webhooki je ne prejmejo.

Uporabite preverjanje HMAC podpisa, da zagotovite avtentičnost payloadov.

## Orodja za testiranje

Za pregled dohodnih webhook payloadov med razvojem lahko uporabite orodja, kot sta [webhook.site](https://webhook.site) ali [ngrok](https://ngrok.com).

## Vrste dogodkov

- **Create Event**: Sproži se, ko je ustvarjen nov komentar.
- **Update Event**: Sproži se, ko je komentar urejen.
- **Delete Event**: Sproži se, ko je komentar izbrisan.

Vsak webhook je povezan z enim dogodkom in eno HTTP metodo (POST, PUT ali DELETE). Vsak dogodek vključuje celotne podatke komentarja v telesu zahteve (glejte [Data Structures](/guide-webhooks.html#webhooks-structures) za format payloada).