## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| userId | string | हाँ |  |
| id | string | हाँ |  |
| changeTicketStateBody | ChangeTicketStateBody | हाँ |  |

## प्रतिक्रिया

रिटर्न करता है: [`ChangeTicketStateResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/ChangeTicketStateResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'changeTicketState उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto body = std::make_shared<ChangeTicketStateBody>();
body->state = U("closed");
body->comment = boost::optional<utility::string_t>(U("Ticket resolved"));
api->changeTicketState(U("my-tenant-123"), U("user@example.com"), U("ticket-456"), *body)
    .then([](std::shared_ptr<ChangeTicketStateResponse>) {});
[inline-code-end]