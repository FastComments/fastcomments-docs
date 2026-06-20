## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| commentId | string | Ne |  |
| externalId | string | Ne |  |
| eventType | string | Ne |  |
| type | string | Ne |  |
| domain | string | Ne |  |
| attemptCountGT | double | Ne |  |

## Odgovor

Vrne: [`GetPendingWebhookEventCountResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetPendingWebhookEventCountResponse.h)

## Primer

[inline-code-attrs-start title = 'getPendingWebhookEventCount Primer'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = utility::conversions::to_string_t("my-tenant-123");
boost::optional<utility::string_t> commentId = boost::optional<utility::string_t>(utility::conversions::to_string_t("cmt-456"));
boost::optional<utility::string_t> externalId = boost::optional<utility::string_t>(utility::conversions::to_string_t("user-42"));
boost::optional<utility::string_t> eventType = boost::optional<utility::string_t>(utility::conversions::to_string_t("comment.created"));
boost::optional<utility::string_t> type = boost::optional<utility::string_t>(utility::conversions::to_string_t("delivery"));
boost::optional<utility::string_t> domain = boost::optional<utility::string_t>(utility::conversions::to_string_t("example.com"));
boost::optional<double> attemptCountGT = boost::optional<double>(2.0);

api->getPendingWebhookEventCount(tenantId, commentId, externalId, eventType, type, domain, attemptCountGT)
.then([](pplx::task<std::shared_ptr<GetPendingWebhookEventCountResponse>> t){
    try {
        auto resp = t.get();
        auto result = resp ? resp : std::make_shared<GetPendingWebhookEventCountResponse>();
        std::cout << "Received pending webhook event response" << std::endl;
    } catch(const std::exception &e) {
        std::cerr << "Error: " << e.what() << std::endl;
    }
});
[inline-code-end]

---