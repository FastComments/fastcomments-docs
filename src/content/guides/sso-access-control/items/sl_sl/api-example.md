Tu bomo korak za korakom prikazali uporabo FastComments API za nastavitev nadzora dostopa.

Preden začnemo, upoštevajte, da ni treba izrecno ustvariti strukture `Group`. Skupine so preprosto identifikatorji, dodani `Users` in `Pages`. Dodajanje skupine uporabniku ali strani samodejno "ustvari" skupino.

Najprej ustvarimo dva uporabnika, `User A` in `User B`; začela bosta v `Group X`:

[inline-code-attrs-start title = 'Primer cURL za ustvarjanje uporabnika A'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Primer cURL za ustvarjanje uporabnika B'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

Zdaj ustvarimo `Page`. Poimenujmo jo `Confidential Page` in doslej nobeden od teh uporabnikov ne bo imel dostopa do nje, saj bo v skupini `CONFIDENTIAL`:

[inline-code-attrs-start title = 'Primer cURL POST za ustvarjanje strani'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

Uporabnika A in B trenutno **NIMATA** dostopa do nove strani. Vendar pa, ker sta v isti skupini `GROUP-X`, se lahko med sabo `@mention`.

Posodobimo `User B`, da bo zdaj lahko dostopal do strani:

[inline-code-attrs-start title = 'Primer cURL za posodobitev uporabnika B'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "groupIds": ["GROUP-X", "CONFIDENTIAL"]
}'
[inline-code-end]

`User B` zdaj pripada obema skupinama. Naši uporabniki se lahko še vedno med sabo `@mention`ajo, vendar si našo zaupno stran lahko ogleda samo `User B`.

Poskrbimo, da bo `User B` lahko ogledal samo zaupno stran:

[inline-code-attrs-start title = 'Primer cURL za posodobitev uporabnika B'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "groupIds": ["CONFIDENTIAL"]
}'
[inline-code-end]

Zdaj lahko ogleda zaupno stran, vendar se noben od naših uporabnikov ne more več medsebojno `@mention`ati, saj so v različnih skupinah.

Vendar pa bo vsak uporabnik, ki ni del nadzora dostopa, **lahko dostopal do naše strani**. Da to preprečite, poskrbite, da noben SSO uporabnik nima `groupIds` nastavljenih na null. Na primer, ustvarimo `User C`, ki ima dostop do vsega:

[inline-code-attrs-start title = 'Primer cURL za ustvarjanje uporabnika C'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

S tem, ko `groupIds` nastavite na null, pomeni, da niso omejeni z nadzorom dostopa.

Zdaj ustvarimo stran, do katere ima dostop vsak:

[inline-code-attrs-start title = 'Primer cURL POST za ustvarjanje strani'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

S tem, ko `accessibleByGroupIds` nastavite na null, rečemo, da ta `Page` ni nadzorovana z nadzorom dostopa, in oba uporabnika lahko dostopita do nje.

S tem zaključujemo pregled uporabe API za nadzor dostopa.