## Παράμετροι

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Όχι |  |
| bulk_create_hash_tags_body | models::BulkCreateHashTagsBody | Όχι |  |

## Απάντηση

Επιστρέφει: [`AddHashTagsBulk200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/add_hash_tags_bulk_200_response.rs)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα add_hash_tags_bulk'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: AddHashTagsBulkParams = AddHashTagsBulkParams {
        tenant_id: Some("acme-corp-tenant".to_string()),
        bulk_create_hash_tags_body: Some(models::BulkCreateHashTagsBody {
            tags: vec![
                models::BulkCreateHashTagsBodyTagsInner {
                    name: "news/article".to_string(),
                    path: "news/article".to_string(),
                    description: Some("Article tag for front page".to_string()),
                    enabled: Some(true),
                },
            ],
        }),
    };

    let response: AddHashTagsBulk200Response = add_hash_tags_bulk(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---