## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenant_id | String | 任意 |  |
| bulk_create_hash_tags_body | models::BulkCreateHashTagsBody | 任意 |  |

## レスポンス

戻り値: [`AddHashTagsBulk200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/add_hash_tags_bulk_200_response.rs)

## 例

[inline-code-attrs-start title = 'add_hash_tags_bulk の例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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