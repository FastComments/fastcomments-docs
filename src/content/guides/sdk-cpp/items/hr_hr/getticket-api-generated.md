## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| id | string | Da |  |
| userId | string | Ne |  |

## Odgovor

Vraća: [`GetTicket_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetTicket_200_response.h)

## Primjer

[inline-code-attrs-start title = 'getTicket Primjer'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t ticketId = U("ticket-456");
boost::optional<utility::string_t> userId = boost::optional<utility::string_t>(U("user@example.com"));
auto ticketResult = std::make_shared<GetTicket_200_response>();
api->getTicket(tenantId, ticketId, userId)
    .then([&ticketResult](pplx::task<std::shared_ptr<GetTicket_200_response>> t) {
        try {
            auto resp = t.get();
            if (resp) ticketResult = resp;
        } catch (...) {}
    });
[inline-code-end]

---