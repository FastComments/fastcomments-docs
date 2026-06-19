## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| id | string | Oui |  |

## Réponse

Renvoie : [`GetModeratorResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetModeratorResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple de getModerator'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-enterprises-72';
const id: string = 'mod_4b2f9a';
const response: GetModeratorResponse = await getModerator(tenantId, id);
const status: APIStatus | undefined = response.status;
const moderator: Moderator | undefined = response.moderator;
const moderatorEmail: string | undefined = response.moderator?.email;
[inline-code-end]

---