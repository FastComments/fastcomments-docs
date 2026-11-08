किसी विशिष्ट कमेंट के लिए नोटिफिकेशन सक्षम या अक्षम करें।

## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| notificationId | string | हाँ |  |
| optedInOrOut | string | हाँ |  |
| commentId | string | हाँ |  |
| sso | string | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`UpdateUserNotificationCommentSubscriptionStatusResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/UpdateUserNotificationCommentSubscriptionStatusResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'updateUserNotificationCommentSubscriptionStatus उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto updateTask = api->updateUserNotificationCommentSubscriptionStatus(
    utility::conversions::to_string_t("my-tenant-123"),
    utility::conversions::to_string_t("notif-456"),
    utility::conversions::to_string_t("optedIn"),
    utility::conversions::to_string_t("comment-789"),
    boost::optional<utility::string_t>(utility::conversions::to_string_t("sso-token-abc"))
).then([](std::shared_ptr<UpdateUserNotificationCommentSubscriptionStatusResponse> resp){
    (void)resp;
});
[inline-code-end]