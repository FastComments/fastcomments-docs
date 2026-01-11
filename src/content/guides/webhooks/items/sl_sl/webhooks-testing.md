---
V skrbniškem vmesniku Webhooks so gumbi `Send Test Payload` za vsako vrsto dogodka (Create, Update, Delete). Dogodka Create in Update pošljeta lažni WebhookComment objekt, medtem ko bo test Delete poslal lažno telo zahteve z zgolj ID.

Test bo izvedel dva klica, da preveri kodo odgovora za "uspešen" (correct API Key) in "neuspešen" (invalid API key) scenarij.

Ko test pošlje invalid API key, morate vrniti statusno kodo 401, da test popolnoma uspe. Če ne preverite pravilno vrednosti tokena, se bo prikazala napaka.

To zagotavlja, da pravilno overite zahtevo.
---