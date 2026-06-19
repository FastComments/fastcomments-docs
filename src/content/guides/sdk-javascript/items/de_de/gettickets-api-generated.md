## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| userId | string | Nein |  |
| state | number | Nein |  |
| skip | number | Nein |  |
| limit | number | Nein |  |

## Antwort

Gibt zurück: [`GetTicketsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTicketsResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'getTickets Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-enterprises";
const userId: string | undefined = "u_56321";
const state: number | undefined = 1;
const skip: number = 0;
const limit: number = 50;
const response: GetTicketsResponse = await getTickets(tenantId, userId, state, skip, limit);
[inline-code-end]

---