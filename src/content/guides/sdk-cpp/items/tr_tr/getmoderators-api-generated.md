## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| skip | double | Hayır |  |

## Yanıt

Döndürür: [`GetModeratorsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetModeratorsResponse.h)

## Örnek

[inline-code-attrs-start title = 'getModerators Örneği'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<double> skip = 10.0;
api->getModerators(tenantId, skip)
    .then([=](pplx::task<std::shared_ptr<GetModeratorsResponse>> t) {
        try {
            auto resp = t.get();
            auto safeResp = resp ? resp : std::make_shared<GetModeratorsResponse>();
            (void)safeResp;
        } catch (const std::exception& e) {
            (void)e;
        }
    });
[inline-code-end]

---