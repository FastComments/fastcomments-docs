## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| commentId | string | Non |  |
| sso | string | Non |  |

## Réponse

Renvoie: [`GetUserInternalProfileResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetUserInternalProfileResponse.h)

## Exemple

[inline-code-attrs-start title = 'Exemple getUserInternalProfile'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
boost::optional<utility::string_t> commentId = boost::optional<utility::string_t>(U("cmt-987654"));
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("tenant-42|alice@example.com"));
auto placeholder = std::make_shared<GetUserInternalProfileResponse>();
api->getUserInternalProfile(commentId, sso)
    .then([](pplx::task<std::shared_ptr<GetUserInternalProfileResponse>> task) {
        try {
            auto resp = task.get();
            if (resp) {
                std::cout << "User profile retrieved\n";
            } else {
                std::cout << "No profile found\n";
            }
        } catch (const std::exception& e) {
            std::cerr << "Error: " << e.what() << '\n';
        }
    });
[inline-code-end]

---