---
## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| options | const GetTenantDailyUsagesOptions& | Sì |  |

## Risposta

Restituisce: [`GetTenantDailyUsagesResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetTenantDailyUsagesResponse.h)

## Esempio

[inline-code-attrs-start title = 'getTenantDailyUsages Esempio'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
GetTenantDailyUsagesOptions opts;
opts.startDate = boost::optional<utility::datetime>(utility::datetime::from_string(U("2023-01-01T00:00:00Z")));
opts.endDate   = boost::optional<utility::datetime>(utility::datetime::from_string(U("2023-01-31T23:59:59Z")));
api->getTenantDailyUsages(tenantId, opts).then([](std::shared_ptr<GetTenantDailyUsagesResponse> resp){
    auto result = std::make_shared<GetTenantDailyUsagesResponse>(*resp);
}).wait();
[inline-code-end]

---