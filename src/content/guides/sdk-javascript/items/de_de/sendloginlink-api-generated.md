## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |
| redirectURL | string | Nein |  |

## Antwort

Gibt zurück: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Beispiel

[inline-code-attrs-start title = 'sendLoginLink Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12a9f3b7";
const id: string = "user_84b2c7d1";
const redirectURL: string = "https://app.mycompany.com/welcome?ref=login_email";
const resultWithoutRedirect: FlagCommentPublic200Response = await sendLoginLink(tenantId, id);
const resultWithRedirect: FlagCommentPublic200Response = await sendLoginLink(tenantId, id, redirectURL);
[inline-code-end]

---