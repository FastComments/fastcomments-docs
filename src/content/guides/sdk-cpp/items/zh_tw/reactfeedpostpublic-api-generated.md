## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|------|------|
| tenantId | string | 是 |  |
| postId | string | 是 |  |
| reactBodyParams | ReactBodyParams | 是 |  |
| isUndo | bool | 否 |  |
| broadcastId | string | 否 |  |
| sso | string | 否 |  |

## 回應

回傳：[`ReactFeedPostPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ReactFeedPostPublic_200_response.h)

## 範例

[inline-code-attrs-start title = 'reactFeedPostPublic 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t postId = U("post-987");
ReactBodyParams reactBodyParams;
reactBodyParams.userId = U("jane.doe@example.com");
reactBodyParams.reaction = U("like");
boost::optional<bool> isUndo = boost::optional<bool>(false);
boost::optional<utility::string_t> broadcastId = boost::optional<utility::string_t>(U("broadcast-001"));
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("sso-token-xyz"));
api->reactFeedPostPublic(tenantId, postId, reactBodyParams, isUndo, broadcastId, sso)
    .then([](pplx::task<std::shared_ptr<ReactFeedPostPublic_200_response>> task) {
        try {
            auto resp = task.get();
            auto out = resp ? resp : std::make_shared<ReactFeedPostPublic_200_response>();
            std::cout << "Reaction processed\n";
        } catch (const std::exception &e) {
            std::cerr << "React failed: " << e.what() << '\n';
        }
    });
[inline-code-end]