## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| userId | string | 예 |  |
| createTicketBody | CreateTicketBody | 예 |  |

## 응답

반환: [`CreateTicketResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CreateTicketResponse.h)

## 예시

[inline-code-attrs-start title = 'createTicket 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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
        // 응답을 필요한 대로 사용합니다
    }catch(const std::exception&){
        // 오류 처리
    }
});
[inline-code-end]

---