## パラメータ

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| id | string | はい |  |
| userId | string | いいえ |  |

## レスポンス

返却: [`GetTicket_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetTicket_200_response.h)

## 例

[inline-code-attrs-start title = 'getTicket の例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t ticketId = U("ticket-456");
boost::optional<utility::string_t> userId = boost::optional<utility::string_t>(U("user@example.com"));
api->getTicket(tenantId, ticketId, userId)
.then([](pplx::task<std::shared_ptr<GetTicket_200_response>> t) {
    try {
        auto resp = t.get();
        auto wrapped = std::make_shared<std::shared_ptr<GetTicket_200_response>>(resp);
        if (*wrapped) {
            std::cout << "Ticket retrieved\n";
        } else {
            std::cout << "No ticket\n";
        }
    } catch (const std::exception &e) {
        std::cerr << "Error: " << e.what() << '\n';
    }
});
[inline-code-end]

---