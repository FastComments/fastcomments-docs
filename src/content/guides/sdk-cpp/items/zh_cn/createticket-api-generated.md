## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| userId | string | 是 |  |
| createTicketBody | CreateTicketBody | 是 |  |

## 响应

返回: [`CreateTicketResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CreateTicketResponse.h)

## 示例

[inline-code-attrs-start title = 'createTicket 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto userId = utility::conversions::to_string_t("john.doe@example.com");
CreateTicketBody ticketBody;
ticketBody.setSubject(utility::conversions::to_string_t("Login Issue"));
ticketBody.setDescription(utility::conversions::to_string_t("Cannot log in after password reset."));
boost::optional<int> priority = 2;
ticketBody.setPriority(priority);
api->createTicket(tenantId, userId, ticketBody).then([](pplx::task<std::shared_ptr<CreateTicketResponse>> task){
    try{
        auto response = task.get();
        // 根据需要使用响应
    }catch(const std::exception&){
        // 处理错误
    }
});
[inline-code-end]

---