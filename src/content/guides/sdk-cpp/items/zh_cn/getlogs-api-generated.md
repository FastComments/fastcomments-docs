## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|------|------|
| commentId | string | 是 |  |
| sso | string | 否 |  |

## 响应

返回: [`ModerationAPIGetLogsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ModerationAPIGetLogsResponse.h)

## 示例

[inline-code-attrs-start title = 'getLogs 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t commentId = U("cmt-987654321");
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("user@example.com"));
api->getLogs(commentId, sso)
.then([](std::shared_ptr<ModerationAPIGetLogsResponse> resp){
    if (!resp) return;
    auto localCopy = std::make_shared<ModerationAPIGetLogsResponse>(*resp);
}).wait();
[inline-code-end]