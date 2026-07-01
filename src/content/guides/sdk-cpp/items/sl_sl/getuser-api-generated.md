## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| id | string | Da |  |

## Odgovor

Vrne: [`GetUserResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetUserResponse.h)

## Primer

[inline-code-attrs-start title = 'Primer getUser'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto userId = utility::conversions::to_string_t("user-789");
boost::optional<utility::string_t> optTag = boost::none;

api->getUser(tenantId, userId)
    .then([=](pplx::task<std::shared_ptr<GetUserResponse>> task) {
        try {
            auto response = task.get();
            if (!response) {
                response = std::make_shared<GetUserResponse>();
            }
            // obdelaj odgovor po potrebi
        } catch (const std::exception&) {
            // obdelaj napako
        }
    });
[inline-code-end]