## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| userId | string | 예 |  |
| id | string | 예 |  |
| changeTicketStateBody | ChangeTicketStateBody | 예 |  |

## 응답

반환: [`ChangeTicketState_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ChangeTicketState_200_response.h)

## 예제

[inline-code-attrs-start title = 'changeTicketState 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t userId = U("agent@example.com");
utility::string_t id = U("TICKET-456");
auto changeBody = std::make_shared<ChangeTicketStateBody>();
changeBody->state = U("resolved");
changeBody->comment = U("Resolved as duplicate after review");
changeBody->notify = boost::optional<bool>(true);
api->changeTicketState(tenantId, userId, id, *changeBody)
.then([](pplx::task<std::shared_ptr<ChangeTicketState_200_response>> t){
    try {
        auto resp = t.get();
        if (resp) {
            auto copy = std::make_shared<ChangeTicketState_200_response>(*resp);
        }
    } catch (const std::exception&) {}
});
[inline-code-end]

---