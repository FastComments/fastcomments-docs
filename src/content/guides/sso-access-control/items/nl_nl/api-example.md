Hier lopen we door het aanroepen van de FastComments API om toegangscontrole in te stellen.

Voordat we beginnen: houd er rekening mee dat we niet expliciet een `Group`-structuur hoeven aan te maken. Groepen zijn gewoon identificatoren
toegevoegd aan `Users` en `Pages`. Het toevoegen van een groep aan een gebruiker of pagina "maakt" de groep automatisch aan.

Eerst maken we twee gebruikers aan, `User A` en `User B`; we zetten ze aanvankelijk in `Group X`:

[inline-code-attrs-start title = 'Voorbeeld cURL: Maak gebruiker A'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Voorbeeld cURL: Maak gebruiker B'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

Nu maken we een `Page` aan. We noemen het onze `Confidential Page`, en tot nu toe heeft geen van deze gebruikers toegang omdat de pagina in de groep `CONFIDENTIAL` zal zitten:

[inline-code-attrs-start title = 'Voorbeeld cURL: Pagina POST'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

Gebruikers A en B hebben op dit moment **GEEN** toegang tot de nieuwe pagina. Omdat ze echter in dezelfde groep, `GROUP-X`, zitten, kunnen ze elkaar `@mention`en.

Laten we `User B` bijwerken zodat die nu toegang tot de pagina heeft:

[inline-code-attrs-start title = 'Voorbeeld cURL: Update gebruiker B'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "groupIds": ["GROUP-X", "CONFIDENTIAL"]
}'
[inline-code-end]

`User B` behoort nu tot beide groepen. Onze gebruikers kunnen elkaar nog steeds `@mention`en, maar alleen `User B` kan onze vertrouwelijke pagina bekijken.

Laten we ervoor zorgen dat `User B` alleen de vertrouwelijke pagina kan bekijken:

[inline-code-attrs-start title = 'Voorbeeld cURL: Update gebruiker B'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "groupIds": ["CONFIDENTIAL"]
}'
[inline-code-end]

Nu kan `User B` de vertrouwelijke pagina bekijken, maar geen van onze gebruikers kan elkaar `@mention`en, omdat ze in verschillende groepen zitten.

Echter, elke gebruiker die geen deel uitmaakt van toegangscontrole **kan onze pagina wel openen**. Om dit te voorkomen, zorg ervoor dat geen SSO-gebruikers hun `groupIds` op null hebben ingesteld. Bijvoorbeeld: laten we `User C` aanmaken, die overal toegang toe heeft:

[inline-code-attrs-start title = 'Voorbeeld cURL: Maak gebruiker C'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

Door `groupIds` op null te zetten, geven we aan dat ze niet beperkt worden door toegangscontrole.

Laten we nu een pagina aanmaken waartoe iedereen toegang heeft:

[inline-code-attrs-start title = 'Voorbeeld cURL: Pagina POST'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

Door `accessibleByGroupIds` op null te zetten, geven we aan dat deze `Page` niet via toegangscontrole wordt geregeld, en beide gebruikers er toegang toe hebben.

Dit rondt onze API-doorloop voor toegangscontrole af.