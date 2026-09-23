## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| tag | string | Да |  |
| deleteHashTagRequestBody | DeleteHashTagRequestBody | Не |  |

## Отговор

Връща: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Пример

[inline-code-attrs-start title = 'deleteHashTag Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_42";
const tag: string = "high-priority";

const requestBody: DeleteHashTagRequestBody = {
  // попълнете полетата при нужда
};

const resultWithBody: APIEmptyResponse = await deleteHashTag(tenantId, tag, requestBody);
const resultWithoutBody: APIEmptyResponse = await deleteHashTag(tenantId, tag);
[inline-code-end]

---