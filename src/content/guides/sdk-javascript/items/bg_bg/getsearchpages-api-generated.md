## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| value | string | Не |  |
| sso | string | Не |  |

## Отговор

Връща: [`ModerationPageSearchResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationPageSearchResponse.ts)

## Пример

[inline-code-attrs-start title = 'getSearchPages Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp-123";
const searchValue: string = "spam";
const ssoToken: string = "sso-abc123def456";

const result1: ModerationPageSearchResponse = await getSearchPages(tenantId);
const result2: ModerationPageSearchResponse = await getSearchPages(tenantId, searchValue);
const result3: ModerationPageSearchResponse = await getSearchPages(tenantId, searchValue, ssoToken);
[inline-code-end]

---