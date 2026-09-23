## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| tenantId | string | Ja |  |
| commentId | string | Ja |  |
| isFlagged | boolean | Ja |  |
| sso | string | Nein |  |

## Antwort

Rückgabe: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'flagCommentPublic Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_42";
const commentId: string = "comment_1001";
const isFlagged: boolean = true;
const sso: string = "sso_user_5678";

const resultWithSso: APIEmptyResponse = await flagCommentPublic(tenantId, commentId, isFlagged, sso);
const resultWithoutSso: APIEmptyResponse = await flagCommentPublic(tenantId, commentId, false);
[inline-code-end]