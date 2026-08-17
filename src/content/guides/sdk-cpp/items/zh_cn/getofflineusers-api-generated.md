Past commenters on the page who are NOT currently online. Sorted by displayName.  
页面上过去的评论者，当前不在线。按 displayName 排序。

Use this after exhausting /users/online to render a "Members" section.  
在耗尽 /users/online 之后使用，以渲染 “Members” 部分。

Cursor pagination on commenterName: server walks the partial {tenantId, urlId, commenterName} index from afterName forward via $gt, no $skip cost.  
在 commenterName 上使用游标分页：服务器从 afterName 开始，通过 $gt 向前遍历部分 {tenantId, urlId, commenterName} 索引，无需 $skip 成本。

## Parameters

| 名称 | 类型 | 必填 | 描述 |
|------|------|------|--------|
| tenantId | string | 是 |  |
| urlId | string | 是 |  |
| options | const GetOfflineUsersOptions& | 是 |  |

## Response

返回: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/PageUsersOfflineResponse.h)

## Example

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

---