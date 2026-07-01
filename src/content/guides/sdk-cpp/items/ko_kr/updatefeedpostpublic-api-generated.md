## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|------|------|
| tenantId | string | 예 |  |
| postId | string | 예 |  |
| updateFeedPostParams | UpdateFeedPostParams | 예 |  |
| options | const UpdateFeedPostPublicOptions& | 예 |  |

## 응답

반환: [`CreateFeedPostResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CreateFeedPostResponse.h)

## 예시

[inline-code-attrs-start title = 'updateFeedPostPublic 예시'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t postId = U("post-456");
UpdateFeedPostParams params;
params.title = boost::optional<utility::string_t>(U("Updated Title"));
params.body = boost::optional<utility::string_t>(U("Updated content of the post."));
UpdateFeedPostPublicOptions options;
options.notifyFollowers = boost::optional<bool>(true);
api->updateFeedPostPublic(tenantId, postId, params, options)
    .then([](std::shared_ptr<CreateFeedPostResponse> resp) {
        auto result = resp;
    });
[inline-code-end]