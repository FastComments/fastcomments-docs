V skrbniku Webhooks so gumbi `Send Test Payload` za vsako vrsto dogodka (Create, Update, Delete). Dogodka Create in Update pošljeta lažen objekt WebhookComment, medtem ko bo pri testiranju Delete poslano lažno telo zahteve z le ID-jem.

## Preverjanje payloadov

Ko testirate integracijo webhooka, preverite, da dohodne zahteve vsebujejo naslednje glave:

1. **`token`** - Vaš API skrivni ključ
2. **`X-FastComments-Timestamp`** - Unix časovni žig (sekunde)
3. **`X-FastComments-Signature`** - HMAC-SHA256 podpis

Uporabite preverjanje HMAC podpisa, da zagotovite pristnost payloadov.

## Orodja za testiranje

Med razvojem lahko uporabite orodja, kot so [webhook.site](https://webhook.site) ali [ngrok](https://ngrok.com), za pregled dohodnih webhook payloadov.

## Vrste dogodkov

- **Create Event**: Sproži se, ko je ustvarjen nov komentar. Privzeta metoda: PUT
- **Update Event**: Sproži se, ko je komentar urejen. Privzeta metoda: PUT
- **Delete Event**: Sproži se, ko je komentar izbrisan. Privzeta metoda: DELETE

Vsak dogodek vključuje celotne podatke komentarja v telesu zahteve (glejte [Strukture podatkov](/guides/webhooks/webhooks-structures) za format payloada).