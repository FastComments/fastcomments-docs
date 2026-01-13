## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | 아니요 |  |
| create_hash_tag_body | models::CreateHashTagBody | 아니요 |  |

## 응답

반환: [`AddHashTag200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/add_hash_tag_200_response.rs)

## 예제

[inline-code-attrs-start title = 'add_hash_tag 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_add_hash_tag(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params: AddHashTagParams = AddHashTagParams {
        tenant_id: Some("acme-corp-tenant".to_string()),
        create_hash_tag_body: Some(models::CreateHashTagBody {
            tag: "breaking-news".to_string(),
            label: Some("Breaking News".to_string()),
            visible: Some(true),
        }),
    };

    let created: AddHashTag200Response = add_hash_tag(configuration, params).await?;
    println!("{:#?}", created);
    Ok(())
}
[inline-code-end]

---