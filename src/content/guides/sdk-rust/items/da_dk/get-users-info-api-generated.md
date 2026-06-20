---
Massebrugeroplysninger for en tenant. Givet userIds, returnér visningsoplysninger fra User / SSOUser.
Bruges af comment widget til at berige brugere, der netop er dukket op via en presence event.
Ingen page context: privatliv håndhæves ensartet (private profiler er maskeret).

## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| ids | String | Ja |  |

## Svar

Returnerer: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/page_users_info_response.rs)

## Eksempel

[inline-code-attrs-start title = 'get_users_info Eksempel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: GetUsersInfoParams = GetUsersInfoParams {
    tenant_id: "acme-corp-tenant".to_string(),
    ids: "alice@example.com,bob@example.com,carol@example.com".to_string(),
    page_size: Some(100),
};
let users_response: PageUsersInfoResponse = get_users_info(&configuration, params).await?;
[inline-code-end]

---