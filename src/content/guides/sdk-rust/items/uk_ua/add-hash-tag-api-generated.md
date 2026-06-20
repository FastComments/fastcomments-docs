---
## Параметри

| Назва | Тип | Обов'язковий | Опис |
|------|------|----------|-------------|
| tenant_id | String | Ні |  |
| create_hash_tag_body | models::CreateHashTagBody | Ні |  |

## Відповідь

Повертає: [`CreateHashTagResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_hash_tag_response.rs)

## Приклад

[inline-code-attrs-start title = 'Приклад add_hash_tag'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: AddHashTagParams = AddHashTagParams {
        tenant_id: Some("acme-corp-tenant".to_string()),
        create_hash_tag_body: Some(models::CreateHashTagBody {
            name: "breaking-news".to_string(),
            slug: "news/breaking".to_string(),
        }),
    };
    let response: CreateHashTagResponse = add_hash_tag(&configuration, params).await?;
    let _created_tag = response;
    Ok(())
}
[inline-code-end]

---