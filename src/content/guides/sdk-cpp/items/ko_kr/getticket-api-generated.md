## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |
| userId | string | 아니오 |  |

## 응답

반환: [`GetTicketResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetTicketResponse.h)

## 예제

[inline-code-attrs-start title = 'getTicket 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t ticketId = U("ticket-456");
boost::optional<utility::string_t> userId = boost::optional<utility::string_t>(U("user@example.com"));
api->getTicket(tenantId, ticketId, userId)
.then([](pplx::task<std::shared_ptr<GetTicketResponse>> task){
    try {
        auto resp = task.get();
        if (resp) {
            auto localCopy = std::make_shared<GetTicketResponse>(*resp);
        }
    } catch (const std::exception&) {
    }
});
[inline-code-end]

---