## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| sso | string | No |  |

## Antwort

Rückgabe: [`GetV1PageLikes`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetV1PageLikes.ts)

## Beispiel

[inline-code-attrs-start title = 'getV1PageLikes Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_12345";
  const urlId: string = "https://myblog.com/post/42";
  const ssoToken: string = "sso_user_987";

  const likesWithSSO: GetV1PageLikes = await getV1PageLikes(tenantId, urlId, ssoToken);
  const likesWithoutSSO: GetV1PageLikes = await getV1PageLikes(tenantId, urlId);
})();
[inline-code-end]