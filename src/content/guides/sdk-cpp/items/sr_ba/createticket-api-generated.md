## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| userId | string | Да |  |
| createTicketBody | CreateTicketBody | Да |  |

## Одговор

Враћа: [`CreateTicket_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CreateTicket_200_response.h)

## Пример

[inline-code-attrs-start title = 'createTicket Пример'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t userId = U("user@example.com");
CreateTicketBody body;
body.subject = U("Checkout failure for order #A123");
body.description = U("Payment declined at gateway for card ending 4242");
body.requesterEmail = userId;
body.priority = boost::optional<int>(2);
body.metadata = boost::optional<utility::string_t>(U("order:A123"));
api->createTicket(tenantId, userId, body).then([](pplx::task<std::shared_ptr<CreateTicket_200_response>> t){
    try {
        auto resp = t.get();
        if (resp) std::cout << "Created ticket id: " << (resp->id.empty() ? U("unknown") : resp->id) << std::endl;
    } catch (const std::exception& e) {
        auto fallback = std::make_shared<CreateTicket_200_response>();
        std::cout << "Failed to create ticket: " << e.what() << std::endl;
    }
});
[inline-code-end]

---