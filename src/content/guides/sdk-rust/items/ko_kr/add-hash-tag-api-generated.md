## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| create_hash_tag_body | models::CreateHashTagBody | No |  |

## 응답

Returns: [`CreateHashTagResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_hash_tag_response.rs)

## 예시

[inline-code-attrs-start title = 'add_hash_tag 예시'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example(cfg: &configuration::Configuration) -> Result<(), Error> {
    let params = AddHashTagParams {
        tenant_id: "acme-corp-tenant".to_string(),
        create_hash_tag_body: Some(models::CreateHashTagBody {
            tag: "news/article".to_string(),
        }),
    };
    let _response: CreateHashTagResponse = add_hash_tag(cfg, params).await?;
    Ok(())
}
[inline-code-end]