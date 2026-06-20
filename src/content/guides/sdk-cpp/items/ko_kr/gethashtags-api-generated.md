## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| page | double | 아니오 |  |

## 응답

반환: [`GetHashTagsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetHashTagsResponse.h)

## 예제

[inline-code-attrs-start title = 'getHashTags 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<double> page = 1.0;
api->getHashTags(tenantId, page).then([](pplx::task<std::shared_ptr<GetHashTagsResponse>> t) {
    try {
        auto resp = t.get();
        if (!resp) resp = std::make_shared<GetHashTagsResponse>();
    } catch (const std::exception&) {
        auto resp = std::make_shared<GetHashTagsResponse>();
    }
});
[inline-code-end]

---