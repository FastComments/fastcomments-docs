## Параметри

| Назва | Тип | Обов'язковий | Опис |
|------|------|--------------|------|
| tenantId | string | Так |  |
| urlId | string | Так |  |
| sso | string | Ні |  |

## Відповідь

Повертає: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## Приклад

[inline-code-attrs-start title = 'putReopenThread Приклад'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
api->putReopenThread(utility::string_t(U("my-tenant-123")), utility::string_t(U("thread-456")), boost::make_optional<utility::string_t>(U("user@example.com")))
    .then([](std::shared_ptr<APIEmptyResponse> result){
        std::cout << "Thread reopened" << std::endl;
    });
[inline-code-end]