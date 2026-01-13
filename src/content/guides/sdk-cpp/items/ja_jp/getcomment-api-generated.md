## パラメータ

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| id | string | はい |  |

## レスポンス

戻り値: [`GetComment_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetComment_200_response.h)

## 例

[inline-code-attrs-start title = 'getComment の例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = utility::conversions::to_string_t("my-tenant-123");
boost::optional<utility::string_t> maybeId = utility::conversions::to_string_t("comment-98765");
auto getTask = api->getComment(tenantId, *maybeId)
    .then([](pplx::task<std::shared_ptr<GetComment_200_response>> t) {
        try {
            auto resp = t.get();
            auto result = resp ? resp : std::make_shared<GetComment_200_response>();
            return result;
        } catch (const std::exception&) {
            return std::make_shared<GetComment_200_response>();
        }
    });
[inline-code-end]

---