## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| page | int32_t | いいえ |  |
| limit | int32_t | いいえ |  |
| skip | int32_t | いいえ |  |
| asTree | bool | いいえ |  |
| skipChildren | int32_t | いいえ |  |
| limitChildren | int32_t | いいえ |  |
| maxTreeDepth | int32_t | いいえ |  |
| urlId | string | いいえ |  |
| userId | string | いいえ |  |
| anonUserId | string | いいえ |  |
| contextUserId | string | いいえ |  |
| hashTag | string | いいえ |  |
| parentId | string | いいえ |  |
| direction | SortDirections | いいえ |  |
| fromDate | int64_t | いいえ |  |
| toDate | int64_t | いいえ |  |

## レスポンス

戻り値: [`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIGetCommentsResponse.h)

## 例

[inline-code-attrs-start title = 'getComments の例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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