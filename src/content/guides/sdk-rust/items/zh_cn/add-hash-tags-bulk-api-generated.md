## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenant_id | String | 否 |  |
| bulk_create_hash_tags_body | models::BulkCreateHashTagsBody | 否 |  |

## 响应

返回：[`BulkCreateHashTagsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/bulk_create_hash_tags_response.rs)

## 示例

[inline-code-attrs-start title = 'add_hash_tags_bulk 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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

---