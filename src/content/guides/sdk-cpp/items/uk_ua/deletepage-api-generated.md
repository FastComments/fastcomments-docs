## Параметри

| Назва | Тип | Обов’язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| id | string | Так |  |

## Відповідь

Повертає: [`DeletePageAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/DeletePageAPIResponse.h)

## Приклад

[inline-code-attrs-start title = 'Приклад deletePage'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto pageId   = utility::conversions::to_string_t("page-456");

api->deletePage(tenantId, pageId)
   .then([](pplx::task<std::shared_ptr<DeletePageAPIResponse>> t) {
       try {
           auto response = t.get();
           // обробити відповідь за потреби
       } catch (const std::exception& ex) {
           // обробити помилку
       }
   });
[inline-code-end]