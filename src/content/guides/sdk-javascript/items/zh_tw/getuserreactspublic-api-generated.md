## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| postIds | Array<string> | No |  |
| sso | string | No |  |

## 回應

回傳：[`UserReactsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UserReactsResponse.ts)

## 範例

[inline-code-attrs-start title = 'getUserReactsPublic 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const postIds: string[] = ["post_abc", "post_def"];
const ssoToken: string = "sso_token_987";

const fullResponse: UserReactsResponse = await getUserReactsPublic(tenantId, postIds, ssoToken);
const minimalResponse: UserReactsResponse = await getUserReactsPublic(tenantId);
[inline-code-end]