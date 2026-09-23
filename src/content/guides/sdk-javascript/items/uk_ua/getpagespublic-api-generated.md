---
Отримати список сторінок для орендаря. Використовується настільним клієнтом FChat для заповнення списку кімнат.  
Потрібно, щоб `enableFChat` було встановлено в true у розв'язаній кастомній конфігурації для кожної сторінки.  
Сторінки, які вимагають SSO, фільтруються за груповим доступом запитуючого користувача.

## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| cursor | string | Ні |  |
| limit | number | Ні |  |
| q | string | Ні |  |
| sortBy | PagesSortBy | Ні |  |
| hasComments | boolean | Ні |  |

## Відповідь

Повертає: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPublicPagesResponse.ts)

## Приклад

[inline-code-attrs-start title = 'getPagesPublic Приклад'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

---