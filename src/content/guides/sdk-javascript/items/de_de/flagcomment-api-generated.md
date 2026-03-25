## Parameters

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |
| userId | string | Nein |  |
| anonUserId | string | Nein |  |

## Response

Gibt zurück: [`FlagComment200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagComment200Response.ts)

## Example

[inline-code-attrs-start title = 'flagComment Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_001';
const id: string = 'comment_7f3a2b9e';
const userId: string = 'user_jdoe_1001';
const anonUserId: string = 'anon_3f2b_visitor';
const result: FlagComment200Response = await flagComment(tenantId, id, userId, anonUserId);
[inline-code-end]

---