## Параметри

| Име | Тип | Задължително | Описание |
|------|------|--------------|----------|
| tenant_id | String | Да |  |
| url_id | String | Да |  |

## Отговор

Връща: `CreateV1PageReact`

## Пример

[inline-code-attrs-start title = 'delete_v1_page_react Пример'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_example(cfg: &configuration::Configuration) -> Result<(), Error> {
    let tenant_id: String = Some("acme-corp-tenant".to_string()).unwrap();
    let url_id: String = "news/article".to_string();
    let params: DeleteV1PageReactParams = DeleteV1PageReactParams {
        tenant_id,
        url_id,
        ..Default::default()
    };
    let _result = delete_v1_page_react(cfg, params).await?;
    Ok(())
}
[inline-code-end]