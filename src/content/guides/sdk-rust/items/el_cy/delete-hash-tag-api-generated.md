## Παράμετροι

| Όνομα | Τύπος | Υποχρεωτικό | Περιγραφή |
|------|------|----------|-------------|
| tag | String | Ναι |  |
| tenant_id | String | Όχι |  |
| delete_hash_tag_request | models::DeleteHashTagRequest | Όχι |  |

## Απόκριση

Επιστρέφει: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα delete_hash_tag'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: DeleteHashTagParams = DeleteHashTagParams {
        tag: "news/politics".to_string(),
        tenant_id: Some("acme-corp-tenant".to_string()),
        delete_hash_tag_request: Some(models::DeleteHashTagRequest::default()),
    };
    let response: FlagCommentPublic200Response = delete_hash_tag(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---