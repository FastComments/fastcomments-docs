## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| textSearch | string | Не |  |
| sso | string | Не |  |

## Отговор

Връща: [`ModerationSuggestResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationSuggestResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример за getSearchSuggest'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant-9f8b7c6d";
  const textSearch: string = "harassment";
  const sso: string = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...";
  const suggestResponse: ModerationSuggestResponse = await getSearchSuggest(tenantId, textSearch, sso);
  const minimalResponse: ModerationSuggestResponse = await getSearchSuggest(tenantId);
})();
[inline-code-end]