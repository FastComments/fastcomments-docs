## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenant_id | String | 否 |  |
| bulk_create_hash_tags_body | models::BulkCreateHashTagsBody | 否 |  |

## 响应

返回: [`AddHashTagsBulk200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/add_hash_tags_bulk_200_response.rs)

## 示例

[inline-code-attrs-start title = 'add_hash_tags_bulk 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn add_tags_example() -> Result<(), Error> {
    let body = BulkCreateHashTagsBody {
        tags: vec![
            BulkCreateHashTagsBodyTagsInner {
                tag: "news/article".to_string(),
                path: "site/news".to_string(),
                description: Some("Articles and press releases".to_string()),
                is_active: Some(true),
                custom_config: Some(CustomConfigParameters { score: Some(0.85) }),
            },
            BulkCreateHashTagsBodyTagsInner {
                tag: "product/launch".to_string(),
                path: "site/products".to_string(),
                description: Some("New product launches".to_string()),
                is_active: Some(true),
                custom_config: Some(CustomConfigParameters { score: Some(0.95) }),
            },
        ],
    };
    let params: AddHashTagsBulkParams = AddHashTagsBulkParams {
        tenant_id: Some("acme-corp-tenant".to_string()),
        bulk_create_hash_tags_body: Some(body),
    };
    let response: AddHashTagsBulk200Response = add_hash_tags_bulk(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---