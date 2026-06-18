## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| id | string | はい |  |
| redirectURL | string | いいえ |  |

## レスポンス

戻り値: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## 例

[inline-code-attrs-start title = 'sendLoginLink の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_0a1b2c3d";
const id: string = "user_984321";
const redirectURL: string = "https://app.acme-corp.com/welcome";
const responseWithRedirect: FlagCommentPublic200Response = await sendLoginLink(tenantId, id, redirectURL);
const responseWithoutRedirect: FlagCommentPublic200Response = await sendLoginLink(tenantId, id);
[inline-code-end]