页面上当前不在线的历史评论者。按 displayName 排序。
在用尽 /users/online 之后，使用此接口渲染“成员”部分。
基于 commenterName 的游标分页：服务器通过 $gt 从 afterName 开始向前遍历部分索引 {tenantId, urlId, commenterName}，没有 $skip 的开销。

## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| urlId | string | 是 |  |
| options | const GetOfflineUsersOptions& | 是 |  |

## 响应

返回：[`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/PageUsersOfflineResponse.h)

## 示例

[inline-code-attrs-start title = 'getOfflineUsers 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t urlId = U("page-456");
GetOfflineUsersOptions options;
options.limit = boost::optional<int>(50);
options.includeDetails = boost::optional<bool>(true);

api->getOfflineUsers(tenantId, urlId, options)
    .then([](pplx::task<std::shared_ptr<PageUsersOfflineResponse>> t) {
        try {
            auto response = t.get();
        } catch (const std::exception&) {
        }
    });
[inline-code-end]