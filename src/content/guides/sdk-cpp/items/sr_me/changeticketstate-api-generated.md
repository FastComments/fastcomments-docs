## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| userId | string | Da |  |
| id | string | Da |  |
| changeTicketStateBody | ChangeTicketStateBody | Da |  |

## Odgovor

Vraća: [`ChangeTicketState_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ChangeTicketState_200_response.h)

## Primjer

[inline-code-attrs-start title = 'Primjer changeTicketState'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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