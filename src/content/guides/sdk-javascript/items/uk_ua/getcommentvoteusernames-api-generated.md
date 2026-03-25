## Параметри

| Назва | Type | Обов'язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| commentId | string | Так |  |
| dir | number | Так |  |
| sso | string | Ні |  |

## Відповідь

Повертає: [`GetCommentVoteUserNames200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentVoteUserNames200Response.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад getCommentVoteUserNames'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'tenant_4f2c1e';
  const commentId: string = 'cmt_9a7b3d';
  const dir: number = 1;
  const resultUpvotes: GetCommentVoteUserNames200Response = await getCommentVoteUserNames(tenantId, commentId, dir);
  const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.fakepayload.signature';
  const dirDown: number = -1;
  const resultDownvotes: GetCommentVoteUserNames200Response = await getCommentVoteUserNames(tenantId, commentId, dirDown, sso);
  console.log(resultUpvotes, resultDownvotes);
})();
[inline-code-end]

---