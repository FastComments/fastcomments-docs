## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| postId | string | Sì |  |
| reactBodyParams | ReactBodyParams | Sì |  |
| isUndo | boolean | No |  |
| broadcastId | string | No |  |
| sso | string | No |  |

## Risposta

Restituisce: [`ReactFeedPostPublicResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ReactFeedPostPublicResponse.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio reactFeedPostPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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