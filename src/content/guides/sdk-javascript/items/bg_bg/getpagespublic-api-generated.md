Списък на страниците за даден tenant. Използва се от FChat десктоп клиента за попълване на списъка му със стаи.
Изисква `enableFChat` да е true в резолвнатата персонализирана конфигурация за всяка страница.
Страниците, изискващи SSO, се филтрират спрямо достъпа на групата на заявяващия потребител.

## Параметри

| Име | Тип | Задължителен | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| cursor | string | Не |  |
| limit | number | Не |  |
| q | string | Не |  |
| sortBy | PagesSortBy | Не |  |
| hasComments | boolean | Не |  |

## Отговор

Връща: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPublicPagesResponse.ts)

## Пример

[inline-code-attrs-start title = 'getPagesPublic Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchPublicPages() {
  const tenantId: string = "tenant_12345";
  const cursor: string = "page_5";
  const limit: number = 20;
  const query: string = "support";
  const hasComments: boolean = true;

  const response: GetPublicPagesResponse = await getPagesPublic(
    tenantId,
    cursor,
    limit,
    query,
    undefined,
    hasComments
  );

  console.log(response);
}
[inline-code-end]