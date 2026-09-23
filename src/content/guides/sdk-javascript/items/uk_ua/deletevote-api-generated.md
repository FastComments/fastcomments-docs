## Параметри

| Назва | Тип | Обов’язковий | Опис |
|------|------|--------------|------|
| tenantId | string | Так |  |
| id | string | Так |  |
| editKey | string | Ні |  |

## Відповідь

Повертає: [`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/VoteDeleteResponse.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад deleteVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function run() {
  const tenantId: string = "tenant_12345";
  const commentId: string = "comment_98765";
  const editKey: string = "edit_abcde";

  const responseWithKey: VoteDeleteResponse = await deleteVote(tenantId, commentId, editKey);
  const responseWithoutKey: VoteDeleteResponse = await deleteVote(tenantId, commentId);
}

run();
[inline-code-end]

---