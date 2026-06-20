---
## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| skip | double | Ne |  |

## Odgovor

Vraća: [`GetQuestionConfigsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetQuestionConfigsResponse.h)

## Primjer

[inline-code-attrs-start title = 'Primjer getQuestionConfigs'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<double> skip = 10.0;
api->getQuestionConfigs(tenantId, skip)
.then([](pplx::task<std::shared_ptr<GetQuestionConfigsResponse>> task){
    try {
        auto resp = task.get();
        auto finalResp = resp ? resp : std::make_shared<GetQuestionConfigsResponse>();
        (void)finalResp;
    } catch (...) {
    }
});
[inline-code-end]

---