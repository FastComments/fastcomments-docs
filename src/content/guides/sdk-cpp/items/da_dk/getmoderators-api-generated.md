## Parametre

| Name | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| skip | double | Nej |  |

## Svar

Returnerer: [`GetModerators_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetModerators_200_response.h)

## Eksempel

[inline-code-attrs-start title = 'Eksempel på getModerators'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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