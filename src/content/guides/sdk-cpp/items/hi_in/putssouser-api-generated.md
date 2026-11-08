## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| id | string | हाँ |  |
| updateAPISSOUserData | UpdateAPISSOUserData | हाँ |  |
| updateComments | bool | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`PutSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/PutSSOUserAPIResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'putSSOUser उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
UpdateAPISSOUserData userData;
userData.email = utility::conversions::to_string_t("alice@example.com");
userData.first_name = utility::conversions::to_string_t("Alice");
userData.last_name = utility::conversions::to_string_t("Smith");
userData.role = utility::conversions::to_string_t("moderator");

api->putSSOUser(
    utility::conversions::to_string_t("my-tenant-123"),
    utility::conversions::to_string_t("alice.smith"),
    userData,
    boost::optional<bool>(true)
).then([](pplx::task<std::shared_ptr<PutSSOUserAPIResponse>> t) {
    auto response = t.get();
});
[inline-code-end]