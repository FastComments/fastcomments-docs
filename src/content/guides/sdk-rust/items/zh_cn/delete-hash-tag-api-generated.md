## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| tag | String | 是 |  |
| delete_hash_tag_request_body | models::DeleteHashTagRequestBody | 否 |  |

## 响应

返回：[`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## 示例

[inline-code-attrs-start title = 'delete_hash_tag 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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