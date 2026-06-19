## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| commentId | string | Da |  |
| broadcastId | string | Da |  |
| commentTextUpdateRequest | CommentTextUpdateRequest | Da |  |
| editKey | string | Ne |  |
| sso | string | Ne |  |

## Odgovor

Vraća: [`PublicAPISetCommentTextResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PublicAPISetCommentTextResponse.ts)

## Primjer

[inline-code-attrs-start title = 'Primjer setCommentText'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_42f2a9';
const commentId: string = 'cmt_9b7d3e';
const broadcastId: string = 'brd_live_2026_06_19';
const editKey: string = 'edk_3f8d2c4a9';
const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.ssoPayload.signature';

const mention: CommentUserMentionInfo = { userId: 'user_789', displayName: 'Jordan Mills' };
const hashtag: CommentUserHashTagInfo = { tag: 'product-launch' };

const commentTextUpdateRequest: CommentTextUpdateRequest = {
  text: 'Updated: clarified the timeline and fixed a typo in the earlier comment.',
  mentions: [mention],
  hashtags: [hashtag]
};

const result: PublicAPISetCommentTextResponse = await setCommentText(
  tenantId,
  commentId,
  broadcastId,
  commentTextUpdateRequest,
  editKey,
  sso
);
[inline-code-end]

---