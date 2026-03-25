## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| page | number | Nein |  |
| limit | number | Nein |  |
| skip | number | Nein |  |
| asTree | boolean | Nein |  |
| skipChildren | number | Nein |  |
| limitChildren | number | Nein |  |
| maxTreeDepth | number | Nein |  |
| urlId | string | Nein |  |
| userId | string | Nein |  |
| anonUserId | string | Nein |  |
| contextUserId | string | Nein |  |
| hashTag | string | Nein |  |
| parentId | string | Nein |  |
| direction | SortDirections | Nein |  |

## Antwort

Gibt zurück: [`GetComments200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetComments200Response.ts)

## Beispiel

[inline-code-attrs-start title = 'getComments Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_42';
const response: GetComments200Response = await getComments(
  tenantId,
  1, // Seite
  20, // Limit
  0, // Überspringen
  true, // als Baumstruktur
  1, // Kinder überspringen
  3, // maximale Kinderanzahl
  4, // maximale Baumtiefe
  'articles/2026/new-product-launch', // URL-ID
  'user_7890', // Benutzer-ID
  'anon_4f3b2', // anonyme Benutzer-ID
  undefined, // Kontext-Benutzer-ID
  '#launch', // Hashtag
  undefined // Eltern-ID
);
[inline-code-end]

---