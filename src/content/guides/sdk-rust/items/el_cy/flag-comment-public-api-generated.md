---
## Παράμετροι

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Ναι |  |
| comment_id | String | Ναι |  |
| is_flagged | bool | Ναι |  |
| sso | String | Όχι |  |

## Απόκριση

Επιστρέφει: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## Παράδειγμα

[inline-code-attrs-start title = 'flag_comment_public Παράδειγμα'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_flag_comment() -> Result<(), Error> {
    let params: FlagCommentPublicParams = FlagCommentPublicParams {
        tenant_id: String::from("acme-corp-tenant"),
        comment_id: String::from("comment-89b3"),
        is_flagged: true,
        sso: Some(String::from("sso-uid-7a2f")),
    };

    let _response: ApiEmptyResponse = flag_comment_public(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---