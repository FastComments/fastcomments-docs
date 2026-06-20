מגיבים קודמים בעמוד שאינם מחוברים כעת. ממוינים לפי displayName.
השתמש בזה לאחר שמיצית את /users/online כדי להציג מדור "Members".
דפדוף בסמן על commenterName: השרת סורק את האינדקס החלקי {tenantId, urlId, commenterName}
מהאינדקס החל מ-afterName קדימה באמצעות $gt, ללא עלות של $skip.

## Parameters

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenant_id | String | כן |  |
| url_id | String | כן |  |
| after_name | String | לא |  |
| after_user_id | String | לא |  |

## Response

מחזיר: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/page_users_offline_response.rs)

## Example

[inline-code-attrs-start title = 'דוגמת get_offline_users'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_offline_users() -> Result<PageUsersOfflineResponse, Error> {
    let params: GetOfflineUsersParams = GetOfflineUsersParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: "news/world/today".to_string(),
        after_name: Some("jane.smith".to_string()),
        after_user_id: Some("user-1024".to_string()),
    };
    let response: PageUsersOfflineResponse = get_offline_users(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---