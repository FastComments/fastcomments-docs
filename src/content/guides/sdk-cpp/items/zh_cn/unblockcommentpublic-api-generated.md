## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| publicBlockFromCommentParams | PublicBlockFromCommentParams | Yes |  |
| sso | string | No |  |

## 响应

返回：[`UnblockSuccess`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/UnblockSuccess.h)

## 示例

[inline-code-attrs-start title = 'unBlockCommentPublic 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto blockParams = PublicBlockFromCommentParams();  
blockParams.reason = U("spam");  
api->unBlockCommentPublic(  
    U("my-tenant-123"),  
    U("comment-789"),  
    blockParams,  
    boost::optional<utility::string_t>(U("sso-token-abc"))  
).then([](std::shared_ptr<UnblockSuccess> result){ });
[inline-code-end]