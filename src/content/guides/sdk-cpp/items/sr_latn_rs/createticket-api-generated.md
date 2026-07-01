## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| userId | string | Da |  |
| createTicketBody | CreateTicketBody | Da |  |

## Odgovor

Vraća: [`CreateTicketResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CreateTicketResponse.h)

## Primer

[inline-code-attrs-start title = 'Primer createTicket'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto userId = utility::conversions::to_string_t("john.doe@example.com");
CreateTicketBody ticketBody;
ticketBody.setSubject(utility::conversions::to_string_t("Login Issue"));
ticketBody.setDescription(utility::conversions::to_string_t("Cannot log in after password reset."));
boost::optional<int> priority = 2;
ticketBody.setPriority(priority);
api->createTicket(tenantId, userId, ticketBody).then([](pplx::task<std::shared_ptr<CreateTicketResponse>> task){
    try{
        auto response = task.get();
        // Koristite odgovor po potrebi
    }catch(const std::exception&){
        // Obradite grešku
    }
});
[inline-code-end]

---