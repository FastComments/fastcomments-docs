Zbiorcze informacje o użytkownikach dla najemcy. Dla podanych userIds zwraca informacje wyświetlane pochodzące z User / SSOUser.
Używane przez widżet komentarzy do wzbogacania użytkowników, którzy właśnie pojawili się w wyniku zdarzenia obecności.
Brak kontekstu strony: prywatność jest egzekwowana jednolicie (profile prywatne są maskowane).

## Parametry

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Tak |  |
| ids | String | Tak |  |

## Odpowiedź

Zwraca: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/page_users_info_response.rs)

## Przykład

[inline-code-attrs-start title = 'Przykład get_users_info'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: GetUsersInfoParams = GetUsersInfoParams {
    tenant_id: "acme-corp-tenant".to_string(),
    ids: "alice@example.com,bob@example.com,carol@example.com".to_string(),
    page_size: Some(100),
};
let users_response: PageUsersInfoResponse = get_users_info(&configuration, params).await?;
[inline-code-end]

---