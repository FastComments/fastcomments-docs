## Παράμετροι

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Ναι |  |
| id | String | Ναι |  |
| send_email | String | Όχι |  |

## Απόκριση

Επιστρέφει: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## Παράδειγμα

[inline-code-attrs-start title = 'delete_moderator Παράδειγμα'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run(configuration: &configuration::Configuration) -> Result<FlagCommentPublic200Response, Error> {
    let params: DeleteModeratorParams = DeleteModeratorParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "moderator-1234".to_string(),
        send_email: Some("true".to_string()),
    };
    let response: FlagCommentPublic200Response = delete_moderator(configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---