## 매개변수

| 이름 | 유형 | 필요 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| urlId | string | 예 |  |
| sso | string | 아니오 |  |

## 응답

반환: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## 예제

[inline-code-attrs-start title = 'putCloseThread 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t(U("my-tenant-4321"));
auto urlId = utility::string_t(U("article-9876"));
boost::optional<utility::string_t> sso = boost::make_optional<utility::string_t>(U("user@example.com"));

api->putCloseThread(tenantId, urlId, sso).then([](pplx::task<std::shared_ptr<APIEmptyResponse>> task){
    try{
        auto response = task.get();
    }catch(const std::exception&){
    }
});
[inline-code-end]