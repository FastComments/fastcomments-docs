## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenant_id | String | はい |  |
| id | String | はい |  |
| error_id | String | はい |  |

## レスポンス

戻り値: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## 例

[inline-code-attrs-start title = 'delete_email_template_render_error の例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let error_id_opt: Option<String> = Some("render-failure-9f3b".to_string());
let params: DeleteEmailTemplateRenderErrorParams = DeleteEmailTemplateRenderErrorParams {
    tenant_id: "acme-corp-tenant".to_string(),
    id: "welcome-email".to_string(),
    error_id: error_id_opt.unwrap(),
};
let response: FlagCommentPublic200Response = delete_email_template_render_error(&configuration, params).await?;
[inline-code-end]

---