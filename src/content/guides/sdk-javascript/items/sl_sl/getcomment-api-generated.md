## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Odgovor

Vrne: [`GetCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentResponse.ts)

## Primer

[inline-code-attrs-start title = 'getComment Primer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp";
  const commentId: string = "comment-987654";
  const result: GetCommentResponse = await getComment(tenantId, commentId);
  const badgeInfo: CommentUserBadgeInfo | undefined = result.comment?.user?.badgeInfo;
  console.log(badgeInfo?.label);
})();
[inline-code-end]