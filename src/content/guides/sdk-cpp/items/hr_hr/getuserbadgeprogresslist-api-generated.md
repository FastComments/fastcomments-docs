## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| userId | string | Ne |  |
| limit | double | Ne |  |
| skip | double | Ne |  |

## Odgovor

Vraća: [`APIGetUserBadgeProgressListResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIGetUserBadgeProgressListResponse.h)

## Primjer

[inline-code-attrs-start title = 'getUserBadgeProgressList Primjer'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<utility::string_t> userId = utility::string_t(U("user@example.com"));
boost::optional<double> limit = 50.0;
boost::optional<double> skip = 0.0;
auto defaultResp = std::make_shared<APIGetUserBadgeProgressListResponse>();
api->getUserBadgeProgressList(tenantId, userId, limit, skip)
.then([defaultResp](pplx::task<std::shared_ptr<APIGetUserBadgeProgressListResponse>> t){
    try {
        auto resp = t.get();
        auto finalResp = resp ? resp : defaultResp;
        (void)finalResp;
    } catch (const std::exception& e) {
        std::cerr << "Error: " << e.what() << std::endl;
    }
});
[inline-code-end]

---