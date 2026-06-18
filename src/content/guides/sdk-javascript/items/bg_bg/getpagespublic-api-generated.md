Изброява страници за наемател. Използва се от настолния клиент FChat за попълване на списъка със стаи.
Изисква `enableFChat` да е true в получения персонализиран конфиг за всяка страница.
Страниците, които изискват SSO, се филтрират спрямо груповия достъп на потребителя, който прави заявката.

## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| cursor | string | Не |  |
| limit | number | Не |  |
| q | string | Не |  |
| sortBy | PagesSortBy | Не |  |
| hasComments | boolean | Не |  |

## Отговор

Връща: [`GetPagesPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPagesPublic200Response.ts)

## Пример

[inline-code-attrs-start title = 'Пример getPagesPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_8f3b2c';
const cursor: string = 'eyJwYWdlIjoiMTIwIn0';
const limit: number = 25;
const q: string = 'homepage hero';
const hasComments: boolean = true;

const response: GetPagesPublic200Response = await getPagesPublic(
  tenantId,
  cursor,
  limit,
  q,
  undefined,
  hasComments
);
[inline-code-end]

---