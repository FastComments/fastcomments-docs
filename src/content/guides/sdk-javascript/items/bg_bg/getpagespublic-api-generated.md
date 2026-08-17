Списък на страниците за наемател. Използва се от настолната клиентска програма FChat за попълване на списъка с стаи.  
Изисква `enableFChat` да бъде true в разрешената персонализирана конфигурация за всяка страница.  
Страници, които изискват SSO, се филтрират спрямо груповия достъп на заявилия потребител.

## Parameters

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| cursor | string | No |  |
| limit | number | No |  |
| q | string | No |  |
| sortBy | PagesSortBy | No |  |
| hasComments | boolean | No |  |

## Response

Връща: [`GetPagesPublicResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPagesPublicResponse.ts)

## Example

[inline-code-attrs-start title = 'Пример за getPagesPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchPages() {
  const tenantId: string = "tenant_12345";
  const cursor: string = "nextPageToken";
  const limit: number = 20;
  const q: string = "blog";
  const sortBy: PagesSortBy = "createdAt";
  const hasComments: boolean = true;

  const response: GetPagesPublicResponse = await getPagesPublic(
    tenantId,
    cursor,
    limit,
    q,
    sortBy,
    hasComments
  );

  console.log(response);
}
[inline-code-end]

---