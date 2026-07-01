## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenant_id | String | Так |  |
| bulk_create_hash_tags_body | models::BulkCreateHashTagsBody | Ні |  |

## Відповідь

Повертає: [`BulkCreateHashTagsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/bulk_create_hash_tags_response.rs)

## Приклад

[inline-code-attrs-start title = 'Приклад add_hash_tags_bulk'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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