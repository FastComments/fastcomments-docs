Набраја странице за тенанта. Користи се од стране FChat десктоп клијента за попуњавање листе соба.
Захтева `enableFChat` да буде true у решеној прилагођеној конфигурацији за сваку страницу.
Странице које захтевају SSO се филтрирају у складу са приступом групи корисника који шаље захтев.

## Параметри

| Назив | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| cursor | string | Не |  |
| limit | number | Не |  |
| q | string | Не |  |
| sortBy | PagesSortBy | Не |  |
| hasComments | boolean | Не |  |

## Одговор

Враћа: [`GetPagesPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPagesPublic200Response.ts)

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