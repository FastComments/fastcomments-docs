## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| createCommentParams | CreateCommentParams | 是 |  |
| options | const SaveCommentOptions& | 是 |  |

## 响应

返回: [`APISaveCommentResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APISaveCommentResponse.h)

## 示例

[inline-code-attrs-start title = 'saveComment 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
CreateCommentParams commentParams;
commentParams.body = utility::string_t(U("Great article!"));
commentParams.author = utility::string_t(U("jane.doe@example.com"));
commentParams.parentId = boost::optional<utility::string_t>(utility::string_t(U("parent-789")));

SaveCommentOptions options;
options.preview = boost::optional<bool>(false);

api->saveComment(utility::string_t(U("my-tenant-123")), commentParams, options)
    .then([](std::shared_ptr<APISaveCommentResponse> response) {
        auto commentId = response->commentId;
    });
[inline-code-end]