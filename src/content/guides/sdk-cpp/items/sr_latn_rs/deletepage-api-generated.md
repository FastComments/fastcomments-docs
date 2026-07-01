## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| id | string | Da |  |

## Odgovor

Vraća: [`DeletePageAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/DeletePageAPIResponse.h)

## Primer

[inline-code-attrs-start title = 'deletePage Primer'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto pageId   = utility::conversions::to_string_t("page-456");

api->deletePage(tenantId, pageId)
   .then([](pplx::task<std::shared_ptr<DeletePageAPIResponse>> t) {
       try {
           auto response = t.get();
           // obradi odgovor po potrebi
       } catch (const std::exception& ex) {
           // obradi grešku
       }
   });
[inline-code-end]