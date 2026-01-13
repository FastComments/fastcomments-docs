## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |

## Antwort

Gibt zurück: [`GetVotes_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetVotes_200_response.h)

## Beispiel

[inline-code-attrs-start title = 'getVotes-Beispiel'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
boost::optional<utility::string_t> tenantId = utility::string_t(U("my-tenant-123"));
boost::optional<utility::string_t> urlId = utility::string_t(U("/posts/2025/new-features"));
auto task = api->getVotes(tenantId.value(), urlId.value()).then([](std::shared_ptr<GetVotes_200_response> resp){
    if (resp) {
        auto copy = std::make_shared<GetVotes_200_response>(*resp);
        std::cout << "Fetched votes successfully\n";
    } else {
        std::cout << "No votes response\n";
    }
});
task.wait();
[inline-code-end]