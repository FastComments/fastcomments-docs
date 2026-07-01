## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| id | string | כן |  |

## תגובה

מחזיר: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## דוגמה

[inline-code-attrs-start title = 'דוגמה למחיקת תצורת שאלה'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto configId = utility::conversions::to_string_t("question-config-456");

api->deleteQuestionConfig(tenantId, configId)
    .then([](std::shared_ptr<APIEmptyResponse> resp) {
        // טיפול במחיקה מוצלחת
    })
    .then([](pplx::task<void> t) {
        try {
            t.get();
        } catch (const std::exception&) {
            // טיפול בשגיאה
        }
    });
[inline-code-end]