## Parameter

| Name | Type | Erforderlich | Beschreibung |
|------|------|--------------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |
| sendEmail | string | Nein |  |

## Antwort

Gibt zurück: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Beispiel

[inline-code-attrs-start title = 'deleteModerator Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_4f3b2c9a';
const id: string = 'mod_9c2d1f7b';
const sendEmail: string = 'true';
const response: FlagCommentPublic200Response = await deleteModerator(tenantId, id, sendEmail);
console.log(response);
[inline-code-end]

---