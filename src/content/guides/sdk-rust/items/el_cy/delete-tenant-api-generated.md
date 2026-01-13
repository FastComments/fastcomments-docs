## Παράμετροι

| Όνομα | Τύπος | Απαραίτητο | Περιγραφή |
|------|------|----------|-------------|
| tenant_id | String | Ναι |  |
| id | String | Ναι |  |
| sure | String | Όχι |  |

## Απόκριση

Επιστρέφει: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα delete_tenant'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example_delete_tenant() -> Result<FlagCommentPublic200Response, Error> {
    let params: DeleteTenantParams = DeleteTenantParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "news/article-12345".to_string(),
        sure: Some("confirm".to_string()),
    };
    let response: FlagCommentPublic200Response = delete_tenant(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---