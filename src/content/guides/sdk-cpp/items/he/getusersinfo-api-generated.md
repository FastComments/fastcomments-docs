---
מידע משתמשים במצב גורף עבור שוכר. בהתבסס על userIds, מחזיר מידע תצוגה מ‑User / SSOUser.
משמש את וידג׳ט ההערות להעשיר משתמשים שהופיעו זה עתה באמצעות אירוע נוכחות.
אין הקשר דף: הפרטיות נאכפת באופן אחיד (פרופילים פרטיים מוסתרים).

## Parameters

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| ids | string | כן |  |

## Response

מחזיר: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/PageUsersInfoResponse.h)

## דוגמה

[inline-code-attrs-start title = 'דוגמת getUsersInfo'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t ids = U("alice@example.com,bob@example.com");
boost::optional<utility::string_t> locale = boost::make_optional(U("en-US"));

api->getUsersInfo(tenantId, ids).then([](pplx::task<std::shared_ptr<PageUsersInfoResponse>> t){
    try{
        auto response = t.get();
        // עיבוד תגובה
    }catch(const std::exception&){
        // טיפול בשגיאה
    }
});
[inline-code-end]

---