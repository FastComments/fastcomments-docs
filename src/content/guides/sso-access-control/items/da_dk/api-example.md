Her gennemgår vi, hvordan man kalder FastComments API'et for at opsætte adgangskontrol.

Før vi begynder, bemærk, at vi ikke behøver eksplicit at oprette en `Group`-struktur. Grupper er simpelthen identifikatorer
der føjes til `Users` og `Pages`. At tilføje en gruppe til en bruger eller side automatisk "opretter" gruppen.

Først opretter vi to brugere, `User A` og `User B`, vi starter dem i `Group X`:

[inline-code-attrs-start title = 'Opret Bruger A cURL-eksempel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Opret Bruger B cURL-eksempel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

Lad os nu oprette en `Page`. Vi kalder den vores `Confidential Page`, og indtil videre vil ingen af disse brugere have adgang til den, da den vil være i gruppen `CONFIDENTIAL`:

[inline-code-attrs-start title = 'Side POST cURL-eksempel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

Bruger A og B har i øjeblikket **IKKE** adgang til den nye side. Da de dog er i samme gruppe, `GROUP-X`, kan de `@mention` hinanden.

Lad os opdatere `User B`, så vedkommende nu kan få adgang til siden:

[inline-code-attrs-start title = 'Opdater Bruger B cURL-eksempel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "groupIds": ["GROUP-X", "CONFIDENTIAL"]
}'
[inline-code-end]

`User B` tilhører nu begge grupper. Vores brugere kan stadig `@mention` hinanden, men kun `User B` kan se vores fortrolige side.

Lad os gøre, så `User B` kun kan se den fortrolige side:

[inline-code-attrs-start title = 'Opdater Bruger B cURL-eksempel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "groupIds": ["CONFIDENTIAL"]
}'
[inline-code-end]

Nu kan de se den fortrolige side, men ingen af vores brugere kan `@mention` hinanden, da de er i forskellige grupper.

Dog vil enhver bruger, der ikke er en del af adgangskontrollen, **kunne få adgang til vores side**. For at forhindre dette, sørg for, at ingen SSO-brugere har deres `groupIds` sat til null. For eksempel, lad os oprette `User C`, som har adgang til alt:

[inline-code-attrs-start title = 'Opret Bruger C cURL-eksempel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

Ved at sætte `groupIds` til null angiver vi, at de ikke er begrænset af adgangskontrol.

Lad os nu oprette en side, som alle har adgang til:

[inline-code-attrs-start title = 'Side POST cURL-eksempel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

Ved at sætte `accessibleByGroupIds` til null angiver vi, at denne `Page` ikke er kontrolleret via adgangskontrol, og begge brugere kan få adgang til den.

Dette fuldender vores API-gennemgang af adgangskontrol.