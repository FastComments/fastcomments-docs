## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| id | string | Yes |  |
| title | string | No |  |

## 응답

반환: [`CreateV1PageReact`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CreateV1PageReact.h)

## 예시

[inline-code-attrs-start title = 'createV2PageReact 예시'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
api->createV2PageReact(
    utility::string_t(U("my-tenant-789")),
    utility::string_t(U("https://example.com/articles/12345")),
    utility::string_t(U("user-42")),
    boost::optional<utility::string_t>(U("Helpful"))
).then([](pplx::task<std::shared_ptr<CreateV1PageReact>> task){
    try{
        auto response = task.get();
    }catch(const std::exception&){ }
});
[inline-code-end]