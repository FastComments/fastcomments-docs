## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| urlId | string | 예 |  |
| sso | string | 아니오 |  |

## 응답

반환: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## 예시

[inline-code-attrs-start title = 'putReopenThread 예시'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
api->putReopenThread(utility::string_t(U("my-tenant-123")), utility::string_t(U("thread-456")), boost::make_optional<utility::string_t>(U("user@example.com")))
    .then([](std::shared_ptr<APIEmptyResponse> result){
        std::cout << "Thread reopened" << std::endl;
    });
[inline-code-end]