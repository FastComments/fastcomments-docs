## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| sso | string | No |  |

## 回應

Returns: [`GetV1PageLikes`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetV1PageLikes.ts)

## 範例

[inline-code-attrs-start title = 'getV1PageLikes 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_12345";
  const urlId: string = "https://myblog.com/post/42";
  const ssoToken: string = "sso_user_987";

  const likesWithSSO: GetV1PageLikes = await getV1PageLikes(tenantId, urlId, ssoToken);
  const likesWithoutSSO: GetV1PageLikes = await getV1PageLikes(tenantId, urlId);
})();
[inline-code-end]

---