## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| id | string | Evet |  |

## Yanıt

Döndürür: [`GetCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentResponse.ts)

## Örnek

[inline-code-attrs-start title = 'getComment Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp";
  const commentId: string = "comment-987654";
  const result: GetCommentResponse = await getComment(tenantId, commentId);
  const badgeInfo: CommentUserBadgeInfo | undefined = result.comment?.user?.badgeInfo;
  console.log(badgeInfo?.label);
})();
[inline-code-end]