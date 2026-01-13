## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tag | string | 是 |  |
| tenantId | string | 否 |  |
| deleteHashTagRequest | DeleteHashTag_request | 否 |  |

## 返回

返回: [`FlagCommentPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FlagCommentPublic_200_response.h)

## 示例

[inline-code-attrs-start title = 'deleteHashTag 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tag = U("inappropriate");
boost::optional<utility::string_t> tenantId = boost::optional<utility::string_t>(U("my-tenant-123"));
DeleteHashTag_request req;
boost::optional<DeleteHashTag_request> optReq = boost::optional<DeleteHashTag_request>(req);

api->deleteHashTag(tag, tenantId, optReq)
.then([](std::shared_ptr<FlagCommentPublic_200_response> resp){
    auto out = std::make_shared<FlagCommentPublic_200_response>(*resp);
    std::cout << "Hashtag removed successfully for tenant\n";
    return out;
}).wait();
[inline-code-end]

---