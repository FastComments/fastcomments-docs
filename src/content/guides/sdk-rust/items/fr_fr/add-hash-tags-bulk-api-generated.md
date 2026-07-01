## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| tenant_id | String | Oui |  |
| bulk_create_hash_tags_body | models::BulkCreateHashTagsBody | Non |  |

## Réponse

Renvoie : [`BulkCreateHashTagsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/bulk_create_hash_tags_response.rs)

## Exemple

[inline-code-attrs-start title = 'add_hash_tags_bulk Exemple'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params = AddHashTagsBulkParams {
        tenant_id: "acme-corp-tenant".to_string(),
        bulk_create_hash_tags_body: Some(models::BulkCreateHashTagsBody {
            tags: vec![
                models::BulkCreateHashTagsBodyTagsInner {
                    tag: "news/article".to_string(),
                },
            ],
        }),
    };
    let _response: BulkCreateHashTagsResponse = add_hash_tags_bulk(&configuration, params).await?;
    Ok(())
}
[inline-code-end]