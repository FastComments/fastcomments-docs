## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| page | int32_t | 아니요 |  |
| limit | int32_t | 아니요 |  |
| skip | int32_t | 아니요 |  |
| asTree | bool | 아니요 |  |
| skipChildren | int32_t | 아니요 |  |
| limitChildren | int32_t | 아니요 |  |
| maxTreeDepth | int32_t | 아니요 |  |
| urlId | string | 아니요 |  |
| userId | string | 아니요 |  |
| anonUserId | string | 아니요 |  |
| contextUserId | string | 아니요 |  |
| hashTag | string | 아니요 |  |
| parentId | string | 아니요 |  |
| direction | SortDirections | 아니요 |  |
| fromDate | int64_t | 아니요 |  |
| toDate | int64_t | 아니요 |  |

## 응답

반환: [`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIGetCommentsResponse.h)

## 예제

[inline-code-attrs-start title = 'getComments 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId(U("my-tenant-123"));
boost::optional<int32_t> page = 1;
boost::optional<int32_t> limit = 50;
boost::optional<bool> asTree = true;
boost::optional<utility::string_t> userId = utility::string_t(U("user@example.com"));
boost::optional<int64_t> fromDate = 1622505600LL;
boost::optional<int64_t> toDate = 1625097600LL;

api->getComments(tenantId,
                 page,
                 limit,
                 boost::optional<int32_t>(),
                 asTree,
                 boost::optional<int32_t>(),
                 boost::optional<int32_t>(),
                 boost::optional<int32_t>(),
                 boost::optional<utility::string_t>(),
                 userId,
                 boost::optional<utility::string_t>(),
                 boost::optional<utility::string_t>(),
                 boost::optional<utility::string_t>(),
                 boost::optional<utility::string_t>(),
                 boost::optional<SortDirections>(),
                 fromDate,
                 toDate)
.then([](pplx::task<std::shared_ptr<APIGetCommentsResponse>> t){
    try {
        auto resp = t.get();
        auto holder = std::make_shared<std::shared_ptr<APIGetCommentsResponse>>(resp);
        (void)holder;
    } catch (...) {}
});
[inline-code-end]

---