---
Massendaten-Benutzerinformationen für einen tenant. Gibt bei Angabe von userIds Anzeigeinformationen aus User / SSOUser zurück. Wird vom Kommentar-Widget verwendet, um Benutzer anzureichern, die gerade über ein presence event aufgetaucht sind. Kein Seitenkontext: Datenschutz wird einheitlich durchgesetzt (private Profile werden maskiert).

## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| ids | String | Ja |  |

## Antwort

Gibt zurück: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/page_users_info_response.rs)

## Beispiel

[inline-code-attrs-start title = 'get_users_info Beispiel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: GetUsersInfoParams = GetUsersInfoParams {
    tenant_id: "acme-corp-tenant".to_string(),
    ids: "alice@example.com,bob@example.com,carol@example.com".to_string(),
    page_size: Some(100),
};
let users_response: PageUsersInfoResponse = get_users_info(&configuration, params).await?;
[inline-code-end]

---