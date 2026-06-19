---
## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| id | string | はい |  |

## レスポンス

返却: [`APIGetCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIGetCommentResponse.ts)

## 例

[inline-code-attrs-start title = 'getComment の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-tenant-7f3b';
const commentId: string = 'cmt_8a7f2d4b';
const response: APIGetCommentResponse = await getComment(tenantId, commentId);
const status: APIStatus | undefined = response.status;
const comment: APIComment | undefined = response.comment;
const badges: CommentUserBadgeInfo[] | undefined = comment?.user?.badges;
const hashtags: CommentUserHashTagInfo[] | undefined = comment?.meta?.hashtags;
const mentions: CommentUserMentionInfo[] | undefined = comment?.user?.mentions;
const meta: APICommentBaseMeta | undefined = comment?.meta;
[inline-code-end]

---