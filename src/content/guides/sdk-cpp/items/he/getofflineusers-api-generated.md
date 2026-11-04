מגיבים קודמים בעמוד שאינם מחוברים כרגע. ממוינים לפי displayName.
השתמשו בזה לאחר אזילת /users/online כדי להציג מקטע "Members".
עימוד באמצעות סמן על commenterName: השרת ממשיך לאורך האינדקס החלקי {tenantId, urlId, commenterName} מ-afterName קדימה באמצעות $gt, ללא עלות $skip.

## פרמטרים

| שם | סוג | חובה | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| urlId | string | כן |  |
| options | const GetOfflineUsersOptions& | כן |  |

## תשובה

מחזיר: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/PageUsersOfflineResponse.h)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getOfflineUsers'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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