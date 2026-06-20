Прошли коментатори на страници који НИСУ тренутно онлајн. Сортирано по displayName.
Користите ово након што исцрпите /users/online да бисте приказали секцију "Чланови".
Курсорска пагинација на commenterName: сервер пролази делимични {tenantId, urlId, commenterName}
индекс од afterName унапред преко $gt, без трошка $skip.

## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| url_id | String | Да |  |
| after_name | String | Не |  |
| after_user_id | String | Не |  |

## Одговор

Враћа: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/page_users_offline_response.rs)

## Пример

[inline-code-attrs-start title = 'get_offline_users Пример'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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