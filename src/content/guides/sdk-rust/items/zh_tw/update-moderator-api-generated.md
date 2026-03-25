## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| id | String | 是 |  |
| update_moderator_body | models::UpdateModeratorBody | 是 |  |

## 回應

回傳: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## 範例

[inline-code-attrs-start title = 'update_moderator 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: UpdateModeratorParams = UpdateModeratorParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "moderator-987".to_string(),
        update_moderator_body: models::UpdateModeratorBody {
            username: Some("jane.doe".to_string()),
            email: Some("jane.doe@acme.com".to_string()),
            role: Some("senior_moderator".to_string()),
            active: Some(true),
            notes: Some("Promoted after successful trial period".to_string()),
        },
    };
    let response: FlagCommentPublic200Response = update_moderator(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---