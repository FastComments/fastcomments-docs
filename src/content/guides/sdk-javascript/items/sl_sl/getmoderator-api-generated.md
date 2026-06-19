---
## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| id | string | Da |  |

## Odgovor

Vrača: [`GetModeratorResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetModeratorResponse.ts)

## Primer

[inline-code-attrs-start title = 'Primer getModerator'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-enterprises-72';
const id: string = 'mod_4b2f9a';
const response: GetModeratorResponse = await getModerator(tenantId, id);
const status: APIStatus | undefined = response.status;
const moderator: Moderator | undefined = response.moderator;
const moderatorEmail: string | undefined = response.moderator?.email;
[inline-code-end]

---