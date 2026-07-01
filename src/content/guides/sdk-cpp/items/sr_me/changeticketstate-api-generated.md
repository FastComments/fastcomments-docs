## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| userId | string | Yes |  |
| id | string | Yes |  |
| changeTicketStateBody | ChangeTicketStateBody | Yes |  |

## Odgovor

Vraća: [`ChangeTicketStateResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ChangeTicketStateResponse.h)

## Primjer

[inline-code-attrs-start title = 'Primjer changeTicketState'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto body = std::make_shared<ChangeTicketStateBody>();
body->state = U("closed");
body->comment = boost::optional<utility::string_t>(U("Ticket resolved"));
api->changeTicketState(U("my-tenant-123"), U("user@example.com"), U("ticket-456"), *body)
    .then([](std::shared_ptr<ChangeTicketStateResponse>) {});
[inline-code-end]