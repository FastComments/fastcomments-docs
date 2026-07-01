## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| page | number | No |  |
| count | number | No |  |
| textSearch | string | No |  |
| byIPFromComment | string | No |  |
| filters | string | No |  |
| searchFilters | string | No |  |
| sorts | string | No |  |
| demo | boolean | No |  |
| tenantId | string | No |  |
| sso | string | No |  |

## Отговор

Връща: [`GetApiCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetApiCommentsResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример getApiComments'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function loadComments() {
  const fullResult: GetApiCommentsResponse = await getApiComments(
    2,                     // страница
    25,                    // брой
    "feedback",           // търсене на текст
    "192.168.1.100",      // IP от коментар
    "approved",           // филтри
    "hasReplies",         // филтри за търсене
    "dateDesc",           // сортиране
    false,                // демо
    "tenant-abc123",      // идентификатор на наемател
    "sso-token-xyz"       // sso
  );

  const minimalResult: GetApiCommentsResponse = await getApiComments(undefined, 5);
}
[inline-code-end]