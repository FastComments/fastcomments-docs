Список сторінок для орендаря. Використовується клієнтом FChat для настільних комп’ютерів для заповнення списку кімнат. Потрібно, щоб `enableFChat` було встановлено в **true** у розвʼязаній кастомній конфігурації для кожної сторінки. Сторінки, які вимагають SSO, фільтруються згідно з груповим доступом запитуючого користувача.

## Параметри

| Назва | Тип | Обов’язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| cursor | string | No |  |
| limit | number | No |  |
| q | string | No |  |
| sortBy | PagesSortBy | No |  |
| hasComments | boolean | No |  |

## Відповідь

Повертає: [`GetPagesPublicResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPagesPublicResponse.ts)

## Приклад

[inline-code-attrs-start title = 'getPagesPublic Приклад'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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