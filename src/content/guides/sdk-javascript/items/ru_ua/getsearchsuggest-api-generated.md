## Параметри

| Назва | Тип | Обов’язковий | Опис |
|------|------|----------|-------------|
| textSearch | string | Ні |  |
| tenantId | string | Ні |  |
| sso | string | Ні |  |

## Відповідь

Повертає: [`GetSearchSuggestResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetSearchSuggestResponse.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад getSearchSuggest'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function example(): Promise<void> {
    const term: string = "fastcomments api";
    const tenant: string = "tenant_001";
    const sso: string = "sso_9a8b7c";

    const fullResult: GetSearchSuggestResponse = await getSearchSuggest(term, tenant, sso);
    const partialResult: GetSearchSuggestResponse = await getSearchSuggest(term);
}

example();
[inline-code-end]

---