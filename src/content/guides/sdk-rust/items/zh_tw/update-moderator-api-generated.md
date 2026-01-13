## 參數

| 名稱 | 型別 | 必填 | 說明 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| id | String | 是 |  |
| update_moderator_body | models::UpdateModeratorBody | 是 |  |

## 回應

回傳: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## 範例

[inline-code-attrs-start title = 'update_moderator 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: UpdateModeratorParams = UpdateModeratorParams {
    tenant_id: "acme-corp-tenant".to_string(),
    id: "moderator-12345".to_string(),
    update_moderator_body: models::UpdateModeratorBody {
        username: "jane.doe".to_string(),
        display_name: Some("Jane Doe".to_string()),
        email: Some("jane.doe@acme.com".to_string()),
        is_active: Some(true),
        permissions: Some(vec!["moderate_comments".to_string(), "view_reports".to_string()]),
    },
};
let response: FlagCommentPublic200Response = update_moderator(&configuration, params).await?;
[inline-code-end]

---