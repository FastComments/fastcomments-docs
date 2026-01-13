## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| commentId | string | Nein |  |
| externalId | string | Nein |  |
| eventType | string | Nein |  |
| type | string | Nein |  |
| domain | string | Nein |  |
| attemptCountGT | double | Nein |  |
| skip | double | Nein |  |

## Antwort

Gibt zurück: [`GetPendingWebhookEvents_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetPendingWebhookEvents_200_response.h)

## Beispiel

[inline-code-attrs-start title = 'getPendingWebhookEvents Beispiel'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<utility::string_t> commentId = boost::optional<utility::string_t>(U("cmt-98765"));
boost::optional<utility::string_t> externalId = boost::optional<utility::string_t>(U("ext-abc-01"));
boost::optional<utility::string_t> eventType = boost::optional<utility::string_t>(U("comment_created"));
boost::optional<utility::string_t> type = boost::optional<utility::string_t>(U("webhook"));
boost::optional<utility::string_t> domain = boost::optional<utility::string_t>(U("example.com"));
boost::optional<double> attemptCountGT = boost::optional<double>(2.0);
boost::optional<double> skip = boost::optional<double>(0.0);

api->getPendingWebhookEvents(tenantId, commentId, externalId, eventType, type, domain, attemptCountGT, skip)
.then([](pplx::task<std::shared_ptr<GetPendingWebhookEvents_200_response>> t) {
    try {
        auto resp = t.get();
        auto copy = std::make_shared<GetPendingWebhookEvents_200_response>(*resp);
        (void)copy;
    } catch (const std::exception& e) {
        (void)e;
        auto empty = std::make_shared<GetPendingWebhookEvents_200_response>();
        (void)empty;
    }
});
[inline-code-end]

---