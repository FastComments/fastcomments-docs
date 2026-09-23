## Paramètres

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| urlId | string | Oui |  |
| sso | string | Non |  |

## Réponse

Retourne : [`GetV1PageLikes`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetV1PageLikes.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple getV1PageLikes'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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