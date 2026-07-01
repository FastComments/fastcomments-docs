## Parameter

| Name     | Type   | Required | Beschreibung |
|----------|--------|----------|---------------|
| tenantId | string | Ja       |               |
| id       | string | Ja       |               |

## Antwort

Rückgabe: [`DeletePageAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/DeletePageAPIResponse.h)

## Beispiel

[inline-code-attrs-start title = 'deletePage Beispiel'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto pageId   = utility::conversions::to_string_t("page-456");

api->deletePage(tenantId, pageId)
   .then([](pplx::task<std::shared_ptr<DeletePageAPIResponse>> t) {
       try {
           auto response = t.get();
           // Verarbeite die Antwort nach Bedarf
       } catch (const std::exception& ex) {
           // Fehler behandeln
       }
   });
[inline-code-end]