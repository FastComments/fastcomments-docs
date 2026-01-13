Ovdje ćemo proći kroz pozivanje FastComments API-ja kako bismo postavili kontrolu pristupa.

Prije nego što počnemo, imajte na umu da ne moramo eksplicitno stvarati strukturu `Group`. Grupe su jednostavno identifikatori dodani `Users` i `Pages`. Dodavanje grupe korisniku ili stranici automatski "stvara" grupu.

Prvo, kreirat ćemo dva korisnika, `User A` i `User B`, počet ćemo ih u `Group X`:

[inline-code-attrs-start title = 'Primjer cURL zahtjeva za stvaranje korisnika A'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Primjer cURL zahtjeva za stvaranje korisnika B'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

Sada ćemo stvoriti `Page`. Nazvat ćemo ju našom `Confidential Page`, i zasad nijedan od ovih korisnika neće imati pristup jer će ona biti u grupi `CONFIDENTIAL`:

[inline-code-attrs-start title = 'Primjer cURL POST zahtjeva za stranicu'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

Korisnici A i B trenutno **NEMAJU** pristup novoj stranici. Međutim, budući da su u istoj grupi, `GROUP-X`, mogu se međusobno `@mention`-ati.

Ažurirajmo `User B` tako da sada može pristupiti stranici:

[inline-code-attrs-start title = 'Primjer cURL zahtjeva za ažuriranje korisnika B'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "groupIds": ["GROUP-X", "CONFIDENTIAL"]
}'
[inline-code-end]

`User B` sada pripada obje grupe. Naši korisnici se i dalje mogu međusobno `@mention`-ati, ali samo `User B` može vidjeti našu povjerljivu stranicu.

Neka `User B` može gledati samo povjerljivu stranicu:

[inline-code-attrs-start title = 'Primjer cURL zahtjeva za ažuriranje korisnika B'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "groupIds": ["CONFIDENTIAL"]
}'
[inline-code-end]

Sada on može vidjeti povjerljivu stranicu, ali nijedan od naših korisnika se više ne može međusobno `@mention`-ati, jer su u različitim grupama.

Međutim, svaki korisnik koji nije dio kontrole pristupa **će moći pristupiti našoj stranici**. Da to spriječite, osigurajte da nijedan SSO User nema `groupIds` postavljen na null. Na primjer, stvorimo `User C`, koji ima pristup svemu:

[inline-code-attrs-start title = 'Primjer cURL zahtjeva za stvaranje korisnika C'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

Postavljanjem `groupIds` na null, govorimo da oni nisu ograničeni kontrolom pristupa.

Sada, kreirajmo stranicu kojoj svi imaju pristup:

[inline-code-attrs-start title = 'Primjer cURL POST zahtjeva za stranicu'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

Postavljanjem `accessibleByGroupIds` na null, govorimo da ova `Page` nije kontrolirana putem kontrole pristupa, i oba korisnika joj mogu pristupiti.

Ovo završava naš pregled API-ja za kontrolu pristupa.