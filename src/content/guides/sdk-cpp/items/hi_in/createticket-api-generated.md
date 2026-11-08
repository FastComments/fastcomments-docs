## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| userId | string | हाँ |  |
| createTicketBody | CreateTicketBody | हाँ |  |

## प्रतिक्रिया

लौटाता है: [`CreateTicketResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/CreateTicketResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'createTicket उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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
        // प्रतिक्रिया का आवश्यकतानुसार उपयोग करें
    }catch(const std::exception&){
        // त्रुटि को संभालें
    }
});
[inline-code-end]