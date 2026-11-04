מגיבים בעבר בדף שאינם מחוברים כרגע. ממוינים לפי displayName.
השתמשו בזה לאחר אזילת /users/online כדי להציג מקטע "Members".
עימוד באמצעות סמן על commenterName: השרת ממשיך לאורך האינדקס החלקי {tenantId, urlId, commenterName}
מ-afterName קדימה באמצעות $gt, ללא עלות $skip.

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| url_id | String | Yes |  |
| after_name | String | No |  |
| after_user_id | String | No |  |

## Response

מחזיר: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/page_users_offline_response.rs)

## Example

[inline-code-attrs-start title = 'דוגמה ל-get_offline_users'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_offline(config: &configuration::Configuration) -> Result<(), Error> {
    let params = GetOfflineUsersParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: "news/article".to_string(),
        after_name: Some("alice".to_string()),
        after_user_id: Some("user-42".to_string()),
    };
    let _response = get_offline_users(config, params).await?;
    Ok(())
}
[inline-code-end]