## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|------------|-----------|
| tenant_id | String | Yes |  |
| tag | String | Yes |  |
| delete_hash_tag_request_body | models::DeleteHashTagRequestBody | No |  |

## Απάντηση

Επιστρέφει: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## Παράδειγμα

[inline-code-attrs-start title = 'delete_hash_tag Παράδειγμα'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = DeleteHashTagParams {
        tenant_id: "acme-corp-tenant".to_string(),
        tag: "news/article".to_string(),
        delete_hash_tag_request_body: Some(models::DeleteHashTagRequestBody {}),
    };
    delete_hash_tag(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---