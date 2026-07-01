---
## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| id | string | Sì |  |

## Risposta

Restituisce: [`GetCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentResponse.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio getComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp";
  const commentId: string = "comment-987654";
  const result: GetCommentResponse = await getComment(tenantId, commentId);
  const badgeInfo: CommentUserBadgeInfo | undefined = result.comment?.user?.badgeInfo;
  console.log(badgeInfo?.label);
})();
[inline-code-end]

---