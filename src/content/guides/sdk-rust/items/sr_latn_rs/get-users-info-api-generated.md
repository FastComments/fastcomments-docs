Masovne informacije o korisnicima za zakupca. Uz date userIds, vraća prikazne informacije iz User / SSOUser. Koristi se od strane widgeta za komentare da obogati korisnike koji su se upravo pojavili putem događaja prisustva. Bez konteksta stranice: privatnost se primenjuje uniformno (privatni profili su maskirani).

## Parametri

| Ime | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenant_id | String | Da |  |
| ids | String | Da |  |

## Odgovor

Vraća: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/page_users_info_response.rs)

## Primer

[inline-code-attrs-start title = 'Primer get_users_info'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params = GetUsersInfoParams {
    tenant_id: "acme-corp-tenant".to_string(),
    ids: "user-1,user-2".to_string(),
};
let page: PageUsersInfoResponse = get_users_info(&configuration, params).await?;
[inline-code-end]