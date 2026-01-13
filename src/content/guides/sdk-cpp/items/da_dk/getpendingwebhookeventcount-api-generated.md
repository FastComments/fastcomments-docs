## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| commentId | string | Nej |  |
| externalId | string | Nej |  |
| eventType | string | Nej |  |
| type | string | Nej |  |
| domain | string | Nej |  |
| attemptCountGT | double | Nej |  |

## Svar

Returnerer: [`GetPendingWebhookEventCount_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetPendingWebhookEventCount_200_response.h)

## Eksempel

[inline-code-attrs-start title = 'getPendingWebhookEventCount Eksempel'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<utility::string_t> commentId = boost::optional<utility::string_t>(U("cmt-789"));
boost::optional<utility::string_t> externalId = boost::optional<utility::string_t>(U("ext-456"));
boost::optional<utility::string_t> eventType = boost::optional<utility::string_t>(U("comment_created"));
boost::optional<utility::string_t> type = boost::optional<utility::string_t>(U("webhook"));
boost::optional<utility::string_t> domain = boost::optional<utility::string_t>(U("example.com"));
boost::optional<double> attemptCountGT = boost::optional<double>(1.0);
api->getPendingWebhookEventCount(tenantId, commentId, externalId, eventType, type, domain, attemptCountGT)
.then([](pplx::task<std::shared_ptr<GetPendingWebhookEventCount_200_response>> t){
    try {
        auto resp = t.get();
        auto respCopy = std::make_shared<GetPendingWebhookEventCount_200_response>(*resp);
        std::cout << "Pending webhook event count retrieved\n";
    } catch (const std::exception &e) {
        std::cerr << "Failed to get pending count: " << e.what() << '\n';
    }
}).wait();
[inline-code-end]

---