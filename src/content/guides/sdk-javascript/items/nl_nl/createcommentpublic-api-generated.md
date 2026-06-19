## Parameters

| Name | Type | Vereist | Beschrijving |
|------|------|---------|-------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |
| broadcastId | string | Ja |  |
| commentData | CommentData | Ja |  |
| sessionId | string | Nee |  |
| sso | string | Nee |  |

## Response

Retourneert: [`SaveCommentsResponseWithPresence`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SaveCommentsResponseWithPresence.ts)

## Voorbeeld

[inline-code-attrs-start title = 'createCommentPublic Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'acme_media';
  const urlId: string = 'articles/sustainable-tech-2026';
  const broadcastId: string = 'broadcast_video_abc123';
  const sessionId: string = 'sess_9f8b7c2a';
  const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.fakeSignature';
  const mention: CommentUserMentionInfo = { userId: 'user_12345', displayName: 'Jane Doe' };
  const hashtag: CommentUserHashTagInfo = { tag: 'sustainability' };
  const commentData: CommentData = {
    content: 'Great insights — I especially liked the section on energy efficiency.',
    authorId: 'user_98765',
    mentions: [mention],
    hashtags: [hashtag],
    metadata: { sentiment: 'positive' }
  };
  const response: SaveCommentsResponseWithPresence = await createCommentPublic(tenantId, urlId, broadcastId, commentData, sessionId, sso);
  console.log(response);
})();
[inline-code-end]

---