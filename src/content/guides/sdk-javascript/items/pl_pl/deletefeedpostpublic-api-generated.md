## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| tenantId | string | Tak |  |
| postId | string | Tak |  |
| broadcastId | string | Nie |  |
| sso | string | Nie |  |

## Odpowiedź

Zwraca: [`DeleteFeedPostPublicResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteFeedPostPublicResponse.ts)

## Przykład

[inline-code-attrs-start title = 'deleteFeedPostPublic Przykład'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

---