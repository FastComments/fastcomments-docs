## Parametreler

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Yanıt

Döndürür: [`DeletePageAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/DeletePageAPIResponse.h)

## Örnek

[inline-code-attrs-start title = 'deletePage Örneği'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto pageId   = utility::conversions::to_string_t("page-456");

api->deletePage(tenantId, pageId)
   .then([](pplx::task<std::shared_ptr<DeletePageAPIResponse>> t) {
       try {
           auto response = t.get();
           // yanıtı gerektiği gibi işleyin
       } catch (const std::exception& ex) {
           // hatayı ele al
       }
   });
[inline-code-end]