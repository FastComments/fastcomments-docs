## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| userId | string | Не |  |
| state | double | Не |  |
| skip | double | Не |  |
| limit | double | Не |  |

## Отговор

Връща: [`GetTickets_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetTickets_200_response.h)

## Пример

[inline-code-attrs-start title = 'Пример за getTickets'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<utility::string_t> userId = U("user@example.com");
boost::optional<double> state = 1.0;
boost::optional<double> skip = 0.0;
boost::optional<double> limit = 50.0;
pplx::task<std::shared_ptr<GetTickets_200_response>> t = api->getTickets(tenantId, userId, state, skip, limit)
.then([](pplx::task<std::shared_ptr<GetTickets_200_response>> prev){
    try {
        auto resp = prev.get();
        if(!resp) resp = std::make_shared<GetTickets_200_response>();
        std::cout << "Got tickets response: " << (resp ? "received" : "none") << std::endl;
    } catch (const std::exception &e) {
        std::cerr << "getTickets failed: " << e.what() << std::endl;
    }
});
[inline-code-end]

---