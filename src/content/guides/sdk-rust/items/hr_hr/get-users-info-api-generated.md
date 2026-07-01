Masovni podaci o korisnicima za tenant. S obzirom na userIds, vraća prikazne informacije iz User / SSOUser.  
Koristi se od strane widgeta za komentare kako bi se obogatili korisnici koji su se upravo pojavili putem događaja prisutnosti.  
Bez konteksta stranice: privatnost se provodi uniformno (privatni profili su maskirani).

## Parameters

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenant_id | String | Da |  |
| ids | String | Da |  |

## Odgovor

Vraća: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/page_users_info_response.rs)

## Primjer

[inline-code-attrs-start title = 'get_users_info Primjer'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params = GetUsersInfoParams {
    tenant_id: "acme-corp-tenant".to_string(),
    ids: "user-1,user-2".to_string(),
};
let page: PageUsersInfoResponse = get_users_info(&configuration, params).await?;
[inline-code-end]