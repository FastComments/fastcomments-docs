## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| postId | string | Oui |  |
| reactBodyParams | ReactBodyParams | Oui |  |
| isUndo | boolean | Non |  |
| broadcastId | string | Non |  |
| sso | string | Non |  |

## Réponse

Retourne : [`ReactFeedPostPublicResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ReactFeedPostPublicResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple reactFeedPostPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'tenant_12345';
  const postId: string = 'post_98765';
  const reactBodyParams: ReactBodyParams = { reaction: 'thumbs_up' };
  const isUndo: boolean = false;
  const broadcastId: string = 'broadcast_abcde';
  const sso: string = 'sso_token_xyz';

  const response: ReactFeedPostPublicResponse = await reactFeedPostPublic(
    tenantId,
    postId,
    reactBodyParams,
    isUndo,
    broadcastId,
    sso
  );

  console.log(response);
})();
[inline-code-end]