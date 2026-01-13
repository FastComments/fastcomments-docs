## Parámetros

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| skip | double | No |  |

## Respuesta

Devuelve: [`GetModerators_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetModerators_200_response.h)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de getModerators'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<double> skip = boost::optional<double>(10);
auto task = api->getModerators(tenantId, skip)
    .then([](std::shared_ptr<GetModerators_200_response> resp) {
        if (!resp) {
            std::cout << "No moderators returned\n";
            return;
        }
        auto copy = std::make_shared<GetModerators_200_response>(*resp);
        std::cout << "Retrieved moderators\n";
    });
task.wait();
[inline-code-end]

---