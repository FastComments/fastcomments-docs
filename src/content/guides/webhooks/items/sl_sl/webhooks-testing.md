V skrbniškem vmesniku Webhooks so gumbi `Send Test Payload` za vsako vrsto dogodka (Create, Update, Delete). Dogodki Create in Update pošljejo testni objekt WebhookComment, medtem ko bo pri testiranju Delete poslan testni telo zahteve, ki vsebuje le ID.

## Preverjanje obremenitev

Pri testiranju vaše webhook integracije preverite, ali dohodni zahtevki vsebujejo naslednje glave:

1. **`token`** - Vaša API skrivnost
2. **`X-FastComments-Timestamp`** - Unix časovni žig (v sekundah)
3. **`X-FastComments-Signature`** - HMAC-SHA256 podpis

Uporabite preverjanje HMAC podpisa, da zagotovite pristnost podatkov.

## Orodja za testiranje

Med razvojem lahko za pregled vsebine dohodnih webhook zahtevkov uporabite orodja, kot so [webhook.site](https://webhook.site) ali [ngrok](https://ngrok.com).

## Vrste dogodkov

- **Create Event**: Sproži se, ko je ustvarjen nov komentar. Privzeta metoda: PUT
- **Update Event**: Sproži se, ko je komentar urejen. Privzeta metoda: PUT
- **Delete Event**: Sproži se, ko je komentar izbrisan. Privzeta metoda: DELETE

Vsak dogodek vključuje polne podatke komentarja v telesu zahtevka (glejte [Strukture podatkov](/guide-webhooks.html#webhooks-structures) za format podatkov).