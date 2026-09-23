## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|-------------|------|
| tenantId | string | Так |  |
| commentId | string | Так |  |
| dir | number | Так |  |
| sso | string | Ні |  |

## Відповідь

Повертає: [`GetCommentVoteUserNamesSuccessResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentVoteUserNamesSuccessResponse.ts)

## Приклад

[inline-code-attrs-start title = 'getCommentVoteUserNames Приклад'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function example() {
  const tenantId: string = 'tenant-9876';
  const commentId: string = 'comment-abc123';
  const dir: number = 1;
  const ssoToken: string = 'sso-xyz789';

  const resultWithSso: GetCommentVoteUserNamesSuccessResponse = await getCommentVoteUserNames(tenantId, commentId, dir, ssoToken);
  const resultWithoutSso: GetCommentVoteUserNamesSuccessResponse = await getCommentVoteUserNames(tenantId, commentId, dir);
}
[inline-code-end]

---