## Παράμετροι

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |

## Απόκριση

Επιστρέφει: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα delete_email_template'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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