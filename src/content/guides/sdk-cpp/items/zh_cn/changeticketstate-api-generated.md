## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|------|-------------|
| tenantId | string | 是 |  |
| userId | string | 是 |  |
| id | string | 是 |  |
| changeTicketStateBody | ChangeTicketStateBody | 是 |  |

## 响应

返回：[`ChangeTicketStateResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ChangeTicketStateResponse.h)

## 示例

[inline-code-attrs-start title = 'changeTicketState 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t userId = U("support-agent@example.com");
utility::string_t ticketId = U("ticket-98765");
auto bodyPtr = std::make_shared<ChangeTicketStateBody>();
bodyPtr->state = U("closed");
bodyPtr->reason = boost::optional<utility::string_t>(U("Resolved by support team"));
api->changeTicketState(tenantId, userId, ticketId, *bodyPtr)
.then([](pplx::task<std::shared_ptr<ChangeTicketStateResponse>> task) {
    try {
        auto resp = task.get();
        if (resp) {
            std::cout << "Ticket state changed successfully" << std::endl;
        } else {
            std::cout << "No response received" << std::endl;
        }
    } catch (const std::exception &e) {
        std::cerr << "Error changing ticket state: " << e.what() << std::endl;
    }
});
[inline-code-end]

---