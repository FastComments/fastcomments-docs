## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| postId | string | 是 |  |
| updateFeedPostParams | UpdateFeedPostParams | 是 |  |
| broadcastId | string | 否 |  |
| sso | string | 否 |  |

## 回應

返回：[`UpdateFeedPostPublicResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateFeedPostPublicResponse.ts)

## 範例

[inline-code-attrs-start title = 'updateFeedPostPublic 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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