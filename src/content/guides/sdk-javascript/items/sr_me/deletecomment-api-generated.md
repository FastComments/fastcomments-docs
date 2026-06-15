## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Да |  |
| contextUserId | string | Не |  |
| isLive | boolean | Не |  |

## Одговор

Враћа: [`DeleteComment200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteComment200Response.ts)

## Пример

[inline-code-attrs-start title = 'deleteComment Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_84a9f2';
const id: string = 'comment_5f3b21';
const contextUserId: string | undefined = 'user_1122';
const isLive: boolean | undefined = true;

async function run(): Promise<void> {
  const result: DeleteComment200Response = await deleteComment(tenantId, id, contextUserId, isLive);
  console.log(result);
}

run();
[inline-code-end]

---