## Parametri

| Ime | Tip | Zahtevano | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| id | string | Da |  |
| updateQuestionConfigBody | UpdateQuestionConfigBody | Da |  |

## Odgovor

Vrne: [`FlagCommentPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FlagCommentPublic_200_response.h)

## Primer

[inline-code-attrs-start title = 'Primer updateQuestionConfig'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t(U("my-tenant-123"));
auto configId = utility::string_t(U("question-config-456"));
auto body = std::make_shared<UpdateQuestionConfigBody>();
body->enabled = true;
body->threshold = 5;
body->moderatorEmail = utility::string_t(U("moderator@example.com"));
body->notifyUsers = boost::optional<bool>(true);
body->description = boost::optional<utility::string_t>(utility::string_t(U("Tighten question moderation for new users")));

api->updateQuestionConfig(tenantId, configId, *body)
    .then([](pplx::task<std::shared_ptr<FlagCommentPublic_200_response>> t) {
        try {
            auto resp = t.get();
            std::cout << (resp ? "updateQuestionConfig succeeded\n" : "updateQuestionConfig returned null\n");
        } catch (const std::exception& e) {
            std::cerr << "updateQuestionConfig failed: " << e.what() << '\n';
        }
    });
[inline-code-end]

---