## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| redirectURL | string | No |  |

## 응답

반환: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## 예시

[inline-code-attrs-start title = 'sendLoginLink 예시'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
api->sendLoginLink(
    U("my-tenant-123"),
    U("user@example.com"),
    boost::make_optional(U("https://myapp.com/auth/callback"))
).then([](std::shared_ptr<APIEmptyResponse> resp) {
});
[inline-code-end]