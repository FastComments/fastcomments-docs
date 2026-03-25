vereist
tenantId
urlId
userIdWS

## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|---------|-------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |
| userIdWS | string | Ja |  |
| startTime | number | Ja |  |
| endTime | number | Ja |  |

## Antwoord

Retourneert: [`GetEventLog200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEventLog200Response.ts)

## Voorbeeld

[inline-code-attrs-start title = 'getEventLog Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'fastcomments-tenant-01';
const urlId: string = 'article-2026-03-25';
const userIdWS: string | undefined = undefined; // optionele upstream-waarde
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