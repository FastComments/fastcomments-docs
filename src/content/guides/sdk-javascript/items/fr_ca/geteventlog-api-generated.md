req
tenantId
urlId
userIdWS

## Paramètres

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| urlId | string | Oui |  |
| userIdWS | string | Oui |  |
| startTime | number | Oui |  |
| endTime | number | Oui |  |

## Réponse

Renvoie: [`GetEventLog200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEventLog200Response.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple de getEventLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'fastcomments-tenant-01';
const urlId: string = 'article-2026-03-25';
const userIdWS: string | undefined = undefined; // valeur en amont facultative
const startTime: number = Date.parse('2026-03-01T00:00:00Z');
const endTime: number = Date.parse('2026-03-25T23:59:59Z');

const eventLogResponse: GetEventLog200Response = await getEventLog(
  tenantId,
  urlId,
  userIdWS ?? 'ws_user_8b1f',
  startTime,
  endTime
);
[inline-code-end]