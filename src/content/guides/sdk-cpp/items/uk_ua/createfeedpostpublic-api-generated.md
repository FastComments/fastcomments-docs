## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| createFeedPostParams | CreateFeedPostParams | Так |  |
| broadcastId | string | Ні |  |
| sso | string | Ні |  |

## Відповідь

Повертає: [`CreateFeedPostPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CreateFeedPostPublic_200_response.h)

## Приклад

[inline-code-attrs-start title = 'Приклад createFeedPostPublic'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t(U("my-tenant-123"));
auto paramsPtr = std::make_shared<CreateFeedPostParams>();
paramsPtr->title = utility::string_t(U("Weekly Release Notes"));
paramsPtr->content = utility::string_t(U("We've deployed version 2.4.1 with bug fixes and UX improvements."));
paramsPtr->authorEmail = utility::string_t(U("alice@example.com"));
boost::optional<utility::string_t> broadcastId = utility::string_t(U("broadcast-456"));
boost::optional<utility::string_t> sso = utility::string_t(U("sso-token-abc123"));
api->createFeedPostPublic(tenantId, *paramsPtr, broadcastId, sso)
    .then([](pplx::task<std::shared_ptr<CreateFeedPostPublic_200_response>> t) {
        try {
            auto resp = t.get();
            if (resp) std::cout << "Feed post created successfully\n";
        } catch (const std::exception& e) {
            std::cerr << "Failed to create feed post: " << e.what() << '\n';
        }
    });
[inline-code-end]

---