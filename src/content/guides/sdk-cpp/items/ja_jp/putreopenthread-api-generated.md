## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| sso | string | No |  |

## 応答

返却: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## 例

[inline-code-attrs-start title = 'putReopenThread の例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
api->putReopenThread(utility::string_t(U("my-tenant-123")), utility::string_t(U("thread-456")), boost::make_optional<utility::string_t>(U("user@example.com")))
    .then([](std::shared_ptr<APIEmptyResponse> result){
        std::cout << "Thread reopened" << std::endl;
    });
[inline-code-end]