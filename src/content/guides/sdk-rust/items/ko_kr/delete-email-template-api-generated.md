## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| id | String | 예 |  |

## 응답

반환: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## 예제

[inline-code-attrs-start title = 'delete_email_template 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_delete_template() -> Result<(), Error> {
    let params: DeleteEmailTemplateParams = DeleteEmailTemplateParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "welcome-email-template".to_string(),
    };
    let confirm_deletion: Option<bool> = Some(true);
    let response: FlagCommentPublic200Response = delete_email_template(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---