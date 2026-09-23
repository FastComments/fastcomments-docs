## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| postIds | Array<string> | いいえ |  |
| sso | string | いいえ |  |

## レスポンス

返却: [`UserReactsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UserReactsResponse.ts)

## 例

[inline-code-attrs-start title = 'getUserReactsPublic の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const postIds: string[] = ["post_abc", "post_def"];
const ssoToken: string = "sso_token_987";

const fullResponse: UserReactsResponse = await getUserReactsPublic(tenantId, postIds, ssoToken);
const minimalResponse: UserReactsResponse = await getUserReactsPublic(tenantId);
[inline-code-end]

---