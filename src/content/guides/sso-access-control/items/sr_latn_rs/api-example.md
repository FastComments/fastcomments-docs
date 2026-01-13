Ovde ćemo proći kroz pozivanje FastComments API-ja za podešavanje kontrole pristupa.

Pre nego što počnemo, imajte na umu da ne moramo eksplicitno kreirati strukturu `Group`. Grupe su jednostavno identifikatori
dodeljeni `Users` i `Pages`. Dodavanje grupe korisniku ili stranici automatski „kreira“ grupu.

Prvo, hajde da kreiramo dva korisnika, `User A` i `User B`, i smestićemo ih u `Group X`:

[inline-code-attrs-start title = 'Primer cURL zahteva za kreiranje korisnika A'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"id": "user-a",
	"username": "User A",
	"email": "usera@example.com",
    "groupIds": ["GROUP-X"]
}'
[inline-code-end]

[inline-code-attrs-start title = 'Primer cURL zahteva za kreiranje korisnika B'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"id": "user-b",
	"username": "User B",
	"email": "userb@example.com",
    "groupIds": ["GROUP-X"]
}'
[inline-code-end]

Sada hajde da kreiramo `Page`. Nazvaćemo je `Confidential Page`, i do sada nijedan od ovih korisnika neće imati pristup jer će ona biti u grupi `CONFIDENTIAL`:

[inline-code-attrs-start title = 'Primer cURL POST zahteva za kreiranje stranice'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/pages?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"title": "Confidential Page",
	"url": "https://mysite.com/confidential",
	"urlId": "https://mysite.com/confidential",
	"accessibleByGroupIds": ["CONFIDENTIAL"]
}'
[inline-code-end]

Korisnici A i B trenutno **NEMAJU** pristup novoj stranici. Međutim, pošto su u istoj grupi, `GROUP-X`, mogu se međusobno `@mention`-ovati.

Ažurirajmo `User B` tako da sada može pristupiti stranici:

[inline-code-attrs-start title = 'Primer cURL zahteva za ažuriranje korisnika B'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "groupIds": ["GROUP-X", "CONFIDENTIAL"]
}'
[inline-code-end]

`User B` sada pripada obe grupe. Naši korisnici i dalje mogu da se međusobno `@mention`uju, ali samo `User B` može da vidi našu `Confidential Page`.

Učinimo da `User B` može samo da pregleda poverljivu stranicu:

[inline-code-attrs-start title = 'Primer cURL zahteva za ažuriranje korisnika B'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "groupIds": ["CONFIDENTIAL"]
}'
[inline-code-end]

Sada može da pristupi poverljivoj stranici, ali nijedan od naših korisnika ne može da se međusobno `@mention`-uje, jer su u različitim grupama.

Međutim, bilo koji korisnik koji nije obuhvaćen kontrolom pristupa **će moći da pristupi našoj stranici**. Da biste to sprečili, postarajte se da nijedan SSO korisnik nema `groupIds` postavljen na null. Na primer, kreirajmo `User C`, koji ima pristup svemu:

[inline-code-attrs-start title = 'Primer cURL zahteva za kreiranje korisnika C'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"id": "user-c",
	"username": "User C",
	"email": "userc@example.com",
    "groupIds": null
}'
[inline-code-end]

Podesivši `groupIds` na null, označavamo da oni nisu ograničeni kontrolom pristupa.

Sada, hajde da kreiramo stranicu kojoj svi imaju pristup:

[inline-code-attrs-start title = 'Primer cURL POST zahteva za kreiranje stranice'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/pages?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"title": "Public Page",
	"url": "https://mysite.com/public",
	"urlId": "https://mysite.com/public",
	"accessibleByGroupIds": null
}'
[inline-code-end]

Podesivši `accessibleByGroupIds` na null, označavamo da ova `Page` nije kontrolisana putem kontrole pristupa, i oba korisnika mogu da joj pristupe.

Ovo završava našu demonstraciju korišćenja API-ja za kontrolu pristupa.