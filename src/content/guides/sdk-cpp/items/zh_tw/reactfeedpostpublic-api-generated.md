## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| postId | string | 是 |  |
| reactBodyParams | ReactBodyParams | 是 |  |
| options | const ReactFeedPostPublicOptions& | 是 |  |

## 回應

回傳：[`ReactFeedPostResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ReactFeedPostResponse.h)

## 範例

[inline-code-attrs-start title = 'reactFeedPostPublic 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto postId = utility::conversions::to_string_t("post-987");
ReactBodyParams reactBody;
reactBody.reaction = utility::conversions::to_string_t("love");
reactBody.userId = utility::conversions::to_string_t("user@example.com");
ReactFeedPostPublicOptions options;
options.metadata = boost::optional<utility::string_t>(utility::conversions::to_string_t("mobile"));
api->reactFeedPostPublic(tenantId, postId, reactBody, options)
    .then([](std::shared_ptr<ReactFeedPostResponse> resp) {
    });
[inline-code-end]