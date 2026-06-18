## Параметри

| Назва | Тип | Обов'язковий | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| id | string | Так |  |

## Відповідь

Повертає: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад використання deleteEmailTemplate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9c4f1b2a";
const id: string = "emailtmpl_4d2b9a5e";
const requestorNote: string | undefined = undefined; // необов'язкові метадані (не вимагаються функцією)
const result: FlagCommentPublic200Response = await deleteEmailTemplate(tenantId, id);
[inline-code-end]

---