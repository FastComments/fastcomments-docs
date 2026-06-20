## Parametri

| Naziv | Type | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| commentId | string | Ne |  |
| externalId | string | Ne |  |
| eventType | string | Ne |  |
| type | string | Ne |  |
| domain | string | Ne |  |
| attemptCountGT | double | Ne |  |
| skip | double | Ne |  |

## Odgovor

Vraća: [`GetPendingWebhookEventsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetPendingWebhookEventsResponse.h)

## Primjer

[inline-code-attrs-start title = 'getPendingWebhookEvents Primjer'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<utility::string_t> commentId = boost::optional<utility::string_t>{ U("cmt-98765") };
boost::optional<utility::string_t> externalId = boost::optional<utility::string_t>{ U("ext-456") };
boost::optional<utility::string_t> eventType = boost::optional<utility::string_t>{ U("delivery_failed") };
boost::optional<utility::string_t> typeOpt = boost::optional<utility::string_t>{ U("webhook") };
boost::optional<utility::string_t> domain = boost::optional<utility::string_t>{ U("app.example.com") };
boost::optional<double> attemptCountGT = boost::optional<double>{ 1.0 };
boost::optional<double> skip = boost::optional<double>{ 0.0 };
api->getPendingWebhookEvents(tenantId, commentId, externalId, eventType, typeOpt, domain, attemptCountGT, skip)
.then([](pplx::task<std::shared_ptr<GetPendingWebhookEventsResponse>> t){
    try {
        auto resp = t.get();
        auto localCopy = std::make_shared<GetPendingWebhookEventsResponse>(*resp);
        (void)localCopy;
    } catch (const std::exception&) {
    }
});
[inline-code-end]

---