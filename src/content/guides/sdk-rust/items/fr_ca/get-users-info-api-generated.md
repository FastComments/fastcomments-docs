Informations groupées sur les utilisateurs pour un tenant. Étant donné des userIds, retourne les informations d'affichage de User / SSOUser.
Utilisé par le widget de commentaires pour enrichir les utilisateurs qui viennent d'apparaître via un événement de présence.
Aucun contexte de page : la confidentialité est appliquée uniformément (les profils privés sont masqués).

## Paramètres

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Oui |  |
| ids | String | Oui |  |

## Réponse

Retourne: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/page_users_info_response.rs)

## Exemple

[inline-code-attrs-start title = 'Exemple de get_users_info'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: GetUsersInfoParams = GetUsersInfoParams {
    tenant_id: "acme-corp-tenant".to_string(),
    ids: "alice@example.com,bob@example.com,carol@example.com".to_string(),
    page_size: Some(100),
};
let users_response: PageUsersInfoResponse = get_users_info(&configuration, params).await?;
[inline-code-end]

---