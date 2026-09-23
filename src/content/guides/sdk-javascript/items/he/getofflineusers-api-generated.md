Past commenters on the page who are NOT currently online. Sorted by displayName.  
מגיבים קודמים בעמוד שאינם מחוברים כעת. ממוינים לפי displayName.

Use this after exhausting /users/online to render a "Members" section.  
השתמש בזה לאחר שמיצית את /users/online כדי להציג סעיף "Members".

Cursor pagination on commenterName: server walks the partial {tenantId, urlId, commenterName} index from afterName forward via $gt, no $skip cost.  
דפדוף עם סמן על commenterName: השרת הולך על החלקית {tenantId, urlId, commenterName} אינדקס מ‑afterName קדימה באמצעות $gt, ללא עלות $skip.

## Parameters

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| urlId | string | כן |  |
| afterName | string | לא |  |
| afterUserId | string | לא |  |

## Response

מחזיר: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PageUsersOfflineResponse.ts)

## Example

[inline-code-attrs-start title = 'getOfflineUsers דוגמה'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchOfflineUsers(): Promise<void> {
  const tenantId: string = "tenant_12345";
  const urlId: string = "page_9876";
  const afterName: string = "John Doe";
  const afterUserId: string = "user_abc123";

  const offlineResponse: PageUsersOfflineResponse = await getOfflineUsers(
    tenantId,
    urlId,
    afterName,
    afterUserId
  );

  console.log(offlineResponse);
}
[inline-code-end]

---