## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| createModeratorBody | CreateModeratorBody | हाँ |  |

## प्रतिक्रिया

लौटाता है: [`CreateModeratorResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/CreateModeratorResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'createModerator उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
CreateModeratorBody moderatorBody;
moderatorBody.email = utility::conversions::to_string_t("moderator@example.com");
moderatorBody.name = utility::conversions::to_string_t("John Moderator");
moderatorBody.notes = boost::optional<utility::string_t>(utility::conversions::to_string_t("Community moderator"));
api->createModerator(utility::conversions::to_string_t("my-tenant-123"), moderatorBody)
    .then([](std::shared_ptr<CreateModeratorResponse> resp) {});
[inline-code-end]