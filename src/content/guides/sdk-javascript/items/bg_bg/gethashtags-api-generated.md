## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| page | number | Не |  |

## Отговор

Връща: [`GetHashTags200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetHashTags200Response.ts)

## Пример

[inline-code-attrs-start title = 'Пример за getHashTags'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp-7a9f";
  const tagsPage1: GetHashTags200Response = await getHashTags(tenantId);
  const tagsPage2: GetHashTags200Response = await getHashTags(tenantId, 2);
  console.log(tagsPage1, tagsPage2);
})();
[inline-code-end]

---