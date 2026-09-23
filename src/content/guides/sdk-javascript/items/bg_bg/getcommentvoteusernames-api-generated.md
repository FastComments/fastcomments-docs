## Параметри

| Име | Тип | Задължително | Описание |
|------|------|--------------|----------|
| tenantId | string | Да |  |
| commentId | string | Да |  |
| dir | number | Да |  |
| sso | string | Не |  |

## Отговор

Връща: [`GetCommentVoteUserNamesSuccessResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentVoteUserNamesSuccessResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример за getCommentVoteUserNames'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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