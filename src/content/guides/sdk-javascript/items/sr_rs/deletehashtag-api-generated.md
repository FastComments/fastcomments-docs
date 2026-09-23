## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| tag | string | Yes |  |
| deleteHashTagRequestBody | DeleteHashTagRequestBody | No |  |

## Одговор

Враћа: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Пример

[inline-code-attrs-start title = 'deleteHashTag Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_42";
const tag: string = "high-priority";

const requestBody: DeleteHashTagRequestBody = {
  // попуни поља по потреби
};

const resultWithBody: APIEmptyResponse = await deleteHashTag(tenantId, tag, requestBody);
const resultWithoutBody: APIEmptyResponse = await deleteHashTag(tenantId, tag);
[inline-code-end]