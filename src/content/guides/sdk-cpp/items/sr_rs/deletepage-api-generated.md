## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Одговор

Враћа: [`DeletePageAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/DeletePageAPIResponse.h)

## Пример

[inline-code-attrs-start title = 'deletePage Пример'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto pageId   = utility::conversions::to_string_t("page-456");

api->deletePage(tenantId, pageId)
   .then([](pplx::task<std::shared_ptr<DeletePageAPIResponse>> t) {
       try {
           auto response = t.get();
           // обради одговор по потреби
       } catch (const std::exception& ex) {
           // обради грешку
       }
   });
[inline-code-end]