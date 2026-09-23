## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|-------------|-------------|
| tenantId | string | Oui |  |
| postId | string | Oui |  |
| broadcastId | string | Non |  |
| sso | string | Non |  |

## Réponse

Renvoie : [`DeleteFeedPostPublicResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteFeedPostPublicResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple deleteFeedPostPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_12345";
  const postId: string = "post_98765";
  const broadcastId: string = "broadcast_abcde";
  const sso: string = "sso_token_xyz";

  const responseWithOpts: DeleteFeedPostPublicResponse = await deleteFeedPostPublic(
    tenantId,
    postId,
    broadcastId,
    sso
  );

  const responseWithoutOpts: DeleteFeedPostPublicResponse = await deleteFeedPostPublic(
    tenantId,
    postId
  );
})();
[inline-code-end]