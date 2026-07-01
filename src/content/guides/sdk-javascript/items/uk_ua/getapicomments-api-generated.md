## Parameters

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

## Response

Повертає: [`GetApiCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetApiCommentsResponse.ts)

## Example

[inline-code-attrs-start title = 'Приклад getApiComments'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function loadComments() {
  const fullResult: GetApiCommentsResponse = await getApiComments(
    2,                     // сторінка
    25,                    // кількість
    "feedback",           // текстовий пошук
    "192.168.1.100",      // за IP з коментаря
    "approved",           // фільтри
    "hasReplies",         // фільтри пошуку
    "dateDesc",           // сортування
    false,                // демо
    "tenant-abc123",      // ідентифікатор орендаря
    "sso-token-xyz"       // sso
  );

  const minimalResult: GetApiCommentsResponse = await getApiComments(undefined, 5);
}
[inline-code-end]