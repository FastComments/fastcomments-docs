## パラメータ

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| postIds | Array<string> | No |  |
| sso | string | No |  |

## レスポンス

返却値: [`GetUserReactsPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserReactsPublic200Response.ts)

## 例

[inline-code-attrs-start title = 'getUserReactsPublic の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-tenant-8a4d2c";
const postIds: string[] = ["post-645a2f", "post-645a30"];
const sso: string = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyXzEyMyIsImlhdCI6MTY2MTYwMDAwMH0.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c";
const result: GetUserReactsPublic200Response = await getUserReactsPublic(tenantId, postIds, sso);
[inline-code-end]

---