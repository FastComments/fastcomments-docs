Past commenters on the page who are NOT currently online. Sorted by displayName.  
המגיבים הקודמים בעמוד שאינם מחוברים כעת. ממוין לפי displayName.  

Use this after exhausting /users/online to render a "Members" section.  
השתמש בזה אחרי שמיצינו את /users/online כדי להציג סעיף "Members".  

Cursor pagination on commenterName: server walks the partial {tenantId, urlId, commenterName} index from afterName forward via $gt, no $skip cost.  
דפדוף עם סמן על commenterName: השרת נע דרך החלק {tenantId, urlId, commenterName} מהאינדקס אחרי afterName קדימה באמצעות $gt, ללא עלות של $skip.  

## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| options | const GetOfflineUsersOptions& | Yes |  |

## תגובה

מחזיר: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PageUsersOfflineResponse.h)

## דוגמה

[inline-code-attrs-start title = 'דוגמת getOfflineUsers'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t urlId = U("page-456");
GetOfflineUsersOptions options;
options.limit = boost::optional<int>(50);
options.includeDetails = boost::optional<bool>(true);

api->getOfflineUsers(tenantId, urlId, options)
    .then([](pplx::task<std::shared_ptr<PageUsersOfflineResponse>> t) {
        try {
            auto response = t.get();
        } catch (const std::exception&) {
        }
    });
[inline-code-end]