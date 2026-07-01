## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| options | const GetPreBanSummaryOptions& | Yes |  |

## תגובה

מחזיר: [`PreBanSummary`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PreBanSummary.h)

## דוגמה

[inline-code-attrs-start title = 'דוגמת getPreBanSummary'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto commentId = utility::conversions::to_string_t("cmt-456789");
GetPreBanSummaryOptions options;
options.locale = boost::optional<utility::string_t>{utility::conversions::to_string_t("en-US")};

api->getPreBanSummary(tenantId, commentId, options)
    .then([](pplx::task<std::shared_ptr<PreBanSummary>> t) {
        try {
            auto summary = t.get();
            // עיבוד סיכום
        } catch (const std::exception&) {
            // טיפול בשגיאה
        }
    });
[inline-code-end]