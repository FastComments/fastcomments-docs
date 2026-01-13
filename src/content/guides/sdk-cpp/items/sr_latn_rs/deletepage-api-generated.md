## Parametri

| Name | Type | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| id | string | Da |  |

## Odgovor

Vraća: [`DeletePageAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/DeletePageAPIResponse.h)

## Primer

[inline-code-attrs-start title = 'deletePage Primer'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t pageId = U("page-456");
boost::optional<utility::string_t> correlationId = boost::optional<utility::string_t>(U("corr-20251122"));
auto task = api->deletePage(tenantId, pageId)
    .then([correlationId](pplx::task<std::shared_ptr<DeletePageAPIResponse>> prev) -> std::shared_ptr<DeletePageAPIResponse> {
        try {
            auto resp = prev.get();
            if (resp) return resp;
            return std::make_shared<DeletePageAPIResponse>();
        } catch (const std::exception&) {
            return std::make_shared<DeletePageAPIResponse>();
        }
    });
[inline-code-end]

---