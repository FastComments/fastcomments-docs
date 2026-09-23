---  
צופים מקוונים כרגע של דף: אנשים שהחיבור WebSocket שלהם מנוי לדף ברגע זה.  
מחזיר anonCount + totalCount (מנויים ברמת החדר, כולל צופים אנונימיים שאינם נספרים).

## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| afterName | string | No |  |
| afterUserId | string | No |  |

## תגובה

מחזיר: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PageUsersOnlineResponse.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמת getOnlineUsers'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]  
[inline-code-start]  
async function fetchOnlineUsers() {
  const tenantId: string = "tenant_12345";
  const urlId: string = "article-9876";
  const afterName: string | undefined = "john_doe";
  const afterUserId: string | undefined = "user_456";

  const onlineUsers: PageUsersOnlineResponse = await getOnlineUsers(
    tenantId,
    urlId,
    afterName,
    afterUserId
  );

  console.log(onlineUsers);
}

fetchOnlineUsers();
[inline-code-end]

---