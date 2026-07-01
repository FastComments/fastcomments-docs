## Параметри

| Назва | Тип | Обов’язковий | Опис |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| editKey | string | No |  |

## Відповідь

Повертає: [`DeleteVoteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteVoteResponse.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад deleteVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp";
  const commentId: string = "comment-9876";
  const editKey: string = "edit-key-123";

  const resultWithEdit: DeleteVoteResponse = await deleteVote(tenantId, commentId, editKey);
  const resultWithoutEdit: DeleteVoteResponse = await deleteVote(tenantId, commentId);
})();
[inline-code-end]

---