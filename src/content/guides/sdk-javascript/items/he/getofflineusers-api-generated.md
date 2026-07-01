מגיבים קודמים בעמוד שאינם מחוברים כעת. ממוין לפי displayName.  
השתמש בזה לאחר שכבר נצלת /users/online כדי ליצור קטע "Members" section.  
דפדוף בעזרת מצביע על commenterName: השרת פועל על החלקי {tenantId, urlId, commenterName}  
אינדקס מ‑afterName קדימה באמצעות $gt, ללא עלות $skip.

## פרמטרים

| שם | סוג | חובה | תיאור |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| afterName | string | No |  |
| afterUserId | string | No |  |

## תגובה

מחזיר: [`GetOfflineUsersResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetOfflineUsersResponse.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמת getOfflineUsers'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchOfflineUsers(): Promise<void> {
    const tenantId: string = "tenant_12345";
    const urlId: string = "thread_9876";
    const afterName: string = "Jane Smith";
    const afterUserId: string = "user_7f9b3c";

    const offlineUsers: GetOfflineUsersResponse = await getOfflineUsers(
        tenantId,
        urlId,
        afterName,
        afterUserId
    );

    console.log(offlineUsers);
}
[inline-code-end]