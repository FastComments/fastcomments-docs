## Параметры

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| userId | string | Yes |  |
| createTicketBody | CreateTicketBody | Yes |  |

## Ответ

Возвращает: [`CreateTicketResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CreateTicketResponse.h)

## Пример

[inline-code-attrs-start title = 'Пример использования createTicket'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t userId = U("user@example.com");
CreateTicketBody createTicketBody;
createTicketBody.subject = U("Unable to post comment");
createTicketBody.description = U("Submitting a comment results in a spinner and no response on desktop Chrome.");
createTicketBody.priority = boost::optional<utility::string_t>(U("high"));
createTicketBody.requesterEmail = boost::optional<utility::string_t>(U("user@example.com"));
auto context = std::make_shared<utility::string_t>(U("web-portal"));
api->createTicket(tenantId, userId, createTicketBody)
.then([context](pplx::task<std::shared_ptr<CreateTicketResponse>> t){
    try {
        auto resp = t.get();
        if (resp) std::cout << "Ticket created successfully" << std::endl;
    } catch (const std::exception &e) {
        std::cerr << "createTicket failed: " << e.what() << std::endl;
    }
});
[inline-code-end]

---