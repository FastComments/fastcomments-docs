## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| commentId | string | हाँ |  |
| options | const GetPreBanSummaryOptions& | हाँ |  |

## प्रतिक्रिया

रिटर्न करता है: [`PreBanSummary`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/PreBanSummary.h)

## उदाहरण

[inline-code-attrs-start title = 'getPreBanSummary उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto commentId = utility::conversions::to_string_t("cmt-456789");
GetPreBanSummaryOptions options;
options.locale = boost::optional<utility::string_t>{utility::conversions::to_string_t("en-US")};

api->getPreBanSummary(tenantId, commentId, options)
    .then([](pplx::task<std::shared_ptr<PreBanSummary>> t) {
        try {
            auto summary = t.get();
            // summary प्रोसेस करें
        } catch (const std::exception&) {
            // त्रुटि संभालें
        }
    });
[inline-code-end]