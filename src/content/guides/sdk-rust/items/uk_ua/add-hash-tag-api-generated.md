---
## Параметри

| Назва | Тип | Обов'язкове | Опис |
|------|------|----------|-------------|
| tenant_id | String | Ні |  |
| create_hash_tag_body | models::CreateHashTagBody | Ні |  |

## Відповідь

Повертає: [`AddHashTag200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/add_hash_tag_200_response.rs)

## Приклад

[inline-code-attrs-start title = 'add_hash_tag Приклад'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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