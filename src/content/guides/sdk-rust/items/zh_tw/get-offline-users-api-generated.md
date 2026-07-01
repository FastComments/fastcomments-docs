Past commenters on the page who are NOT currently online. Sorted by displayName.  
頁面上過去的評論者（目前未在線），依 displayName 排序。

Use this after exhausting /users/online to render a "Members" section.  
在耗盡 /users/online 後，用於呈現「Members」區段。

Cursor pagination on commenterName: server walks the partial {tenantId, urlId, commenterName} index from afterName forward via $gt, no $skip cost.  
在 commenterName 上的游標分頁：伺服器從 afterName 之後的 {tenantId, urlId, commenterName} 索引走訪，使用 $gt，無 $skip 成本。

## Parameters

| 名稱 | 類型 | 必填 | 描述 |
|------|------|------|------|
| tenant_id | String | 是 |  |
| url_id | String | 是 |  |
| after_name | String | 否 |  |
| after_user_id | String | 否 |  |

## Response

返回：[`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/page_users_offline_response.rs)

## Example

[inline-code-attrs-start title = 'get_offline_users 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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