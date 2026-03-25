## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenant_id | String | はい |  |
| url_id_ws | String | はい |  |
| user_ids | String | はい |  |

## レスポンス

戻り値: [`GetUserPresenceStatuses200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_user_presence_statuses_200_response.rs)

## 例

[inline-code-attrs-start title = 'get_user_presence_statuses の例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<GetUserPresenceStatuses200Response, Error> {
    let tenant_id: String = "acme-corp-tenant".to_string();
    let url_id_ws: String = "news/article".to_string();
    let user_ids_opt: Option<String> = Some("user123,user456,user789".to_string());
    let params = GetUserPresenceStatusesParams {
        tenant_id,
        url_id_ws,
        user_ids: user_ids_opt.unwrap(),
    };
    let presence: GetUserPresenceStatuses200Response = get_user_presence_statuses(&configuration, params).await?;
    Ok(presence)
}
[inline-code-end]

---