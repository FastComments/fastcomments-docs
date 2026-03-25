## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| id | String | 예 |  |
| error_id | String | 예 |  |

## 응답

반환: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## 예제

[inline-code-attrs-start title = 'delete_email_template_render_error 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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