## Параметри

| Назив | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenant_id | String | Не |  |
| bulk_create_hash_tags_body | models::BulkCreateHashTagsBody | Не |  |

## Одговор

Враћа: [`BulkCreateHashTagsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/bulk_create_hash_tags_response.rs)

## Пример

[inline-code-attrs-start title = 'add_hash_tags_bulk Пример'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: AddHashTagsBulkParams = AddHashTagsBulkParams {
        tenant_id: Some("acme-corp-tenant".to_string()),
        bulk_create_hash_tags_body: Some(models::BulkCreateHashTagsBody {
            tags: vec![
                models::BulkCreateHashTagsBodyTagsInner {
                    name: "breaking-news".to_string(),
                    path: "news/breaking".to_string(),
                    custom_config: Some(models::CustomConfigParameters {
                        visibility: Some("public".to_string())
                    })
                },
                models::BulkCreateHashTagsBodyTagsInner {
                    name: "product-launch".to_string(),
                    path: "company/product/launch".to_string(),
                    custom_config: Some(models::CustomConfigParameters {
                        visibility: Some("private".to_string())
                    })
                }
            ]
        })
    };

    let response: BulkCreateHashTagsResponse = add_hash_tags_bulk(&configuration, params).await?;
    println!("{:#?}", response);
    Ok(())
}
[inline-code-end]