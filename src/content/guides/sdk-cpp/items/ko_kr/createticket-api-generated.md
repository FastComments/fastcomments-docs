## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| userId | string | 예 |  |
| createTicketBody | CreateTicketBody | 예 |  |

## 응답

반환: [`CreateTicket_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CreateTicket_200_response.h)

## 예제

[inline-code-attrs-start title = 'createTicket 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t userId = U("user@example.com");
auto bodyPtr = std::make_shared<CreateTicketBody>();
bodyPtr->subject = U("Comments not posting on article");
bodyPtr->message = U("Submitting a comment returns 200 but it does not appear for other users");
bodyPtr->priority = boost::optional<utility::string_t>(U("low"));
api->createTicket(tenantId, userId, *bodyPtr)
.then([](pplx::task<std::shared_ptr<CreateTicket_200_response>> task){
    try {
        auto resp = task.get();
        if (resp) {
            auto created = resp;
        }
    } catch (const std::exception& e) {
        (void)e;
    }
});
[inline-code-end]