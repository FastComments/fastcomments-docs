## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| options | const GetApiCommentsOptions& | 예 |  |

## 응답

반환: [`ModerationAPIGetCommentsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ModerationAPIGetCommentsResponse.h)

## 예제

[inline-code-attrs-start title = 'getApiComments 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto options = GetApiCommentsOptions{};
options.page = boost::make_optional(2);
options.authorEmail = boost::make_optional<utility::string_t>(U("user@example.com"));
options.includeDeleted = boost::make_optional(false);

api->getApiComments(U("my-tenant-123"), options).then([](pplx::task<std::shared_ptr<ModerationAPIGetCommentsResponse>> task){
    try{
        auto response = task.get();
    }catch(const std::exception&){
    }
});
[inline-code-end]