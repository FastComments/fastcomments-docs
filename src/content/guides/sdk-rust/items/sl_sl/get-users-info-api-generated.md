Hromadni podatki o uporabnikih za najemnika. Glede na userIds vrne prikazne podatke iz User / SSOUser.
Uporablja ga pripomoček za komentarje za obogatitev uporabnikov, ki so se pravkar pojavili preko presence event.
Brez konteksta strani: zasebnost se uveljavlja enotno (zasebni profili so zamaskirani).

## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenant_id | String | Da |  |
| ids | String | Da |  |

## Odziv

Vrne: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/page_users_info_response.rs)

## Primer

[inline-code-attrs-start title = 'Primer get_users_info'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: GetUsersInfoParams = GetUsersInfoParams {
    tenant_id: "acme-corp-tenant".to_string(),
    ids: "alice@example.com,bob@example.com,carol@example.com".to_string(),
    page_size: Some(100),
};
let users_response: PageUsersInfoResponse = get_users_info(&configuration, params).await?;
[inline-code-end]

---