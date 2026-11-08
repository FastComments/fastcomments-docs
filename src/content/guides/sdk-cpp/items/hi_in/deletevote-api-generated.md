## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| id | string | हाँ |  |
| editKey | string | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/VoteDeleteResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'deleteVote उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto voteId = utility::conversions::to_string_t("vote-9876");
boost::optional<utility::string_t> editKey = utility::conversions::to_string_t("edit-abc123");

api->deleteVote(tenantId, voteId, editKey).then([](pplx::task<std::shared_ptr<VoteDeleteResponse>> task) {
    try {
        auto response = task.get();
        // प्रतिक्रिया को आवश्यकतानुसार प्रोसेस करें
    } catch (const std::exception&) {
        // त्रुटि को संभालें
    }
});
[inline-code-end]