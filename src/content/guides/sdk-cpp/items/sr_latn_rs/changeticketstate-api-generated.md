## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| userId | string | Da |  |
| id | string | Da |  |
| changeTicketStateBody | ChangeTicketStateBody | Da |  |

## Odgovor

Vraća: [`ChangeTicketStateResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ChangeTicketStateResponse.h)

## Primer

[inline-code-attrs-start title = 'changeTicketState Primer'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto body = std::make_shared<ChangeTicketStateBody>();
body->state = U("closed");
body->comment = boost::optional<utility::string_t>(U("Ticket resolved"));
api->changeTicketState(U("my-tenant-123"), U("user@example.com"), U("ticket-456"), *body)
    .then([](std::shared_ptr<ChangeTicketStateResponse>) {});
[inline-code-end]

---