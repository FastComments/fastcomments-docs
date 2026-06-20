---
## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| url_id | String | Да |  |

## Одговор

Враћа: `CreateV1PageReact`

## Пример

[inline-code-attrs-start title = 'Пример delete_v1_page_react'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_delete_react() -> Result<(), Error> {
    let tenant_id: String = "acme-corp-tenant".to_string();
    let maybe_url_id: Option<String> = Some("news/politics/2026-election".to_string());
    let url_id: String = maybe_url_id.unwrap();
    let params: DeleteV1PageReactParams = DeleteV1PageReactParams { tenant_id, url_id };
    let deleted: CreateV1PageReact = delete_v1_page_react(&configuration, params).await?;
    let _result: CreateV1PageReact = deleted;
    Ok(())
}
[inline-code-end]

---