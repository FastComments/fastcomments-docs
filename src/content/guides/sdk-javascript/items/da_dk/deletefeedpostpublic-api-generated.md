## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| postId | string | Ja |  |
| broadcastId | string | Nej |  |
| sso | string | Nej |  |

## Svar

Returnerer: [`DeleteFeedPostPublicResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteFeedPostPublicResponse.ts)

## Eksempel

[inline-code-attrs-start title = 'deleteFeedPostPublic Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp";
  const postId: string = "post_987654321";
  const broadcastId: string = "broadcast_2024Q1";
  const sso: string = "sso_4fa3b9c2";

  const response1: DeleteFeedPostPublicResponse = await deleteFeedPostPublic(tenantId, postId);
  const response2: DeleteFeedPostPublicResponse = await deleteFeedPostPublic(tenantId, postId, broadcastId);
  const response3: DeleteFeedPostPublicResponse = await deleteFeedPostPublic(tenantId, postId, broadcastId, sso);
})();
[inline-code-end]