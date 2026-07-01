## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| postId | string | Ja |  |
| updateFeedPostParams | UpdateFeedPostParams | Ja |  |
| broadcastId | string | Nej |  |
| sso | string | Nej |  |

## Svar

Returnerer: [`UpdateFeedPostPublicResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateFeedPostPublicResponse.ts)

## Eksempel

[inline-code-attrs-start title = 'updateFeedPostPublic Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function updatePostExample() {
  const tenantId: string = 'tenant-9f8b7c';
  const postId: string = 'post-3a4d5e';
  const updateParams: UpdateFeedPostParams = {
    title: 'New Announcement',
    content: 'We have updated our terms of service.',
    media: [
      {
        url: 'https://cdn.example.com/assets/image.png',
        type: 'image',
        asset: {
          width: 800,
          height: 600,
          size: 124000
        } as FeedPostMediaItemAsset
      } as FeedPostMediaItem
    ],
    link: {
      url: 'https://example.com/terms',
      title: 'Read the new TOS'
    } as FeedPostLink
  };
  const broadcastId: string = 'broadcast-2023-09';
  const sso: string = 'sso-abc123xyz';
  const result: UpdateFeedPostPublicResponse = await updateFeedPostPublic(
    tenantId,
    postId,
    updateParams,
    broadcastId,
    sso
  );
}
[inline-code-end]