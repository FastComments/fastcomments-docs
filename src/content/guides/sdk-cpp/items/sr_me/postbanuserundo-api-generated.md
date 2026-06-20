## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| banUserUndoParams | BanUserUndoParams | Да |  |
| sso | string | Не |  |

## Response

Враћа: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## Пример

[inline-code-attrs-start title = 'postBanUserUndo пример'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto paramsPtr = std::make_shared<BanUserUndoParams>();
paramsPtr->setTenantId(U("my-tenant-123"));
paramsPtr->setUserEmail(U("jane.doe@example.com"));
paramsPtr->setModeratorId(U("mod-456"));
boost::optional<utility::string_t> sso = utility::string_t(U("https://sso.mycompany.com/session-token"));
api->postBanUserUndo(*paramsPtr, sso)
    .then([](pplx::task<std::shared_ptr<APIEmptyResponse>> t) {
        try {
            auto resp = t.get();
            if (resp) std::cout << "Ban undone successfully\n";
            else std::cout << "Ban undo returned empty response\n";
        } catch (const std::exception& e) {
            std::cout << "postBanUserUndo failed: " << e.what() << '\n';
        }
    });
[inline-code-end]