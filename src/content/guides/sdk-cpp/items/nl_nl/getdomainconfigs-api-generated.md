## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |

## Response

Retourneert: [`GetDomainConfigs_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetDomainConfigs_200_response.h)

## Voorbeeld

[inline-code-attrs-start title = 'getDomainConfigs Voorbeeld'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = utility::conversions::to_string_t("my-tenant-123");
boost::optional<utility::string_t> requestId = utility::conversions::to_string_t("req-789");
api->getDomainConfigs(tenantId)
    .then([requestId](pplx::task<std::shared_ptr<GetDomainConfigs_200_response>> t){
        try {
            auto resp = t.get();
            auto copy = std::make_shared<GetDomainConfigs_200_response>(*resp);
            if (requestId) std::cout << "Request id present\n";
            std::cout << "Domain configs loaded\n";
        } catch (const std::exception &e) {
            std::cerr << "Error: " << e.what() << '\n';
        }
    }).wait();
[inline-code-end]

---