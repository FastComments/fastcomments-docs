---
## Odgovor

Vrne: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## Primer

[inline-code-attrs-start title = 'Primer uporabe logoutPublic'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
boost::optional<utility::string_t> tenantId = boost::optional<utility::string_t>(U("my-tenant-123"));
boost::optional<utility::string_t> sessionToken = boost::optional<utility::string_t>(U("user@example.com"));
api->logoutPublic()
.then([](pplx::task<std::shared_ptr<APIEmptyResponse>> task){
    try {
        auto resp = task.get();
        if (resp) {
            std::cout << "Logout successful\n";
            return std::make_shared<APIEmptyResponse>(*resp);
        }
        std::cout << "Logout returned empty response\n";
        return std::make_shared<APIEmptyResponse>();
    } catch (const std::exception& e) {
        std::cout << "Logout failed: " << e.what() << '\n';
        return std::make_shared<APIEmptyResponse>();
    }
}).wait();
[inline-code-end]

---