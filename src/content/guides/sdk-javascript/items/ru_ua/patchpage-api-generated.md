## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Да |  |
| updateAPIPageData | UpdateAPIPageData | Да |  |

## Ответ

Возвращает: [`PatchPageAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PatchPageAPIResponse.ts)

## Пример

[inline-code-attrs-start title = 'patchPage Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_12345";
  const pageId: string = "page_98765";

  const updateData: UpdateAPIPageData = {
    title: "Updated FAQ Page"
    // description?: string может быть опущено
  };

  const response: PatchPageAPIResponse = await patchPage(tenantId, pageId, updateData);
})();
[inline-code-end]