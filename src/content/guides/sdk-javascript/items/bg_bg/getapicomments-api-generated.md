## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| page | number | Не |  |
| count | number | Не |  |
| textSearch | string | Не |  |
| byIPFromComment | string | Не |  |
| filters | string | Не |  |
| searchFilters | string | Не |  |
| sorts | string | Не |  |
| demo | boolean | Не |  |
| sso | string | Не |  |

## Отговор

Връща: [`ModerationAPIGetCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationAPIGetCommentsResponse.ts)

## Пример

[inline-code-attrs-start title = 'getApiComments Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchComments(): Promise<void> {
  const tenantId: string = "tenant_12345";
  const page: number = 2;
  const count: number = 20;
  const textSearch: string = "typescript";
  const byIPFromComment: string = "192.168.1.100";
  const filters: string = "spam,offensive";
  const searchFilters: string = "user:john";
  const sorts: string = "date_desc";
  const demo: boolean = true;
  const sso: string = "sso_token_abc";

  const response: ModerationAPIGetCommentsResponse = await getApiComments(
    tenantId,
    page,
    count,
    textSearch,
    byIPFromComment,
    filters,
    searchFilters,
    sorts,
    demo,
    sso
  );
}

fetchComments();
[inline-code-end]