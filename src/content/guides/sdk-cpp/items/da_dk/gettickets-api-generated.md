## Parametre

| Name | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| userId | string | Nej |  |
| state | double | Nej |  |
| skip | double | Nej |  |
| limit | double | Nej |  |

## Respons

Returnerer: [`GetTickets_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetTickets_200_response.h)

## Eksempel

[inline-code-attrs-start title = 'getTickets Eksempel'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<utility::string_t> userId = boost::optional<utility::string_t>(U("user@example.com"));
boost::optional<double> state = boost::optional<double>(1.0);
boost::optional<double> skip = boost::optional<double>(0.0);
boost::optional<double> limit = boost::optional<double>(50.0);
api->getTickets(tenantId, userId, state, skip, limit)
.then([](pplx::task<std::shared_ptr<GetTickets_200_response>> t){
    try {
        auto resp = t.get();
        auto copy = std::make_shared<GetTickets_200_response>(*resp);
        std::cout << "Retrieved tickets: " << (resp ? "non-null" : "null") << std::endl;
    } catch (const std::exception &e) {
        std::cerr << "getTickets failed: " << e.what() << std::endl;
    }
});
[inline-code-end]