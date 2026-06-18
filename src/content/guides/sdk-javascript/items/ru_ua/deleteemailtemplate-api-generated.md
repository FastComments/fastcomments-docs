## Параметры

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Ответ

Возвращает: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Пример

[inline-code-attrs-start title = 'Пример deleteEmailTemplate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9c4f1b2a";
const id: string = "emailtmpl_4d2b9a5e";
const requestorNote: string | undefined = undefined; // необязательные метаданные (не требуются этой функции)
const result: FlagCommentPublic200Response = await deleteEmailTemplate(tenantId, id);
[inline-code-end]