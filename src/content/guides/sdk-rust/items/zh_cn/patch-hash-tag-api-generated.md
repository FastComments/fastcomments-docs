## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| tag | String | Yes |  |
| update_hash_tag_body | models::UpdateHashTagBody | No |  |

## 响应

返回: [`UpdateHashTagResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/update_hash_tag_response.rs)

## 示例

[inline-code-attrs-start title = 'patch_hash_tag 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = PatchHashTagParams {
        tenant_id: "acme-corp-tenant".into(),
        tag: "news/article".into(),
        update_hash_tag_body: Some(models::UpdateHashTagBody::default()),
    };
    let _response = patch_hash_tag(&configuration, params).await?;
    Ok(())
}
[inline-code-end]