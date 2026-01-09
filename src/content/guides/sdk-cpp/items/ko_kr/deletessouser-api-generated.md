## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |
| deleteComments | bool | 아니요 |  |
| commentDeleteMode | string | 아니요 |  |

## 응답

반환: [`DeleteSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/DeleteSSOUserAPIResponse.h)

## 예제

[inline-code-attrs-start title = 'deleteSSOUser 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t ssoId = U("user@example.com");
boost::optional<bool> deleteComments = true;
boost::optional<utility::string_t> commentDeleteMode = U("anonymize");

api->deleteSSOUser(tenantId, ssoId, deleteComments, commentDeleteMode)
.then([](pplx::task<std::shared_ptr<DeleteSSOUserAPIResponse>> t){
    try {
        auto resp = t.get();
        auto respCopy = std::make_shared<DeleteSSOUserAPIResponse>(*resp);
        (void)respCopy;
    } catch (const std::exception&) {
    }
});
[inline-code-end]

---