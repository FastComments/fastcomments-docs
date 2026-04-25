## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Да |  |
| deleteComments | string | Не |  |
| commentDeleteMode | string | Не |  |

## Отговор

Връща: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Пример

[inline-code-attrs-start title = 'deleteTenantUser Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function run(): Promise<void> {
  const tenantId: string = "acme_corp_tenant_9f1a2b";
  const id: string = "user_4d2a1b6c";
  const deleteComments: string = "true"; // премахнете и коментарите на потребителя
  const commentDeleteMode: string = "permanent"; // "permanent" или "soft"
  const result: FlagCommentPublic200Response = await deleteTenantUser(tenantId, id, deleteComments, commentDeleteMode);
  console.log(result);
}
run();
[inline-code-end]

---