---
Informations groupées sur les utilisateurs pour un tenant. Étant donné des userIds, renvoie les informations d'affichage depuis User / SSOUser.
Utilisé par le widget de commentaires pour enrichir les utilisateurs qui viennent d'apparaître via un événement de présence.
Pas de contexte de page : la confidentialité est appliquée de manière uniforme (les profils privés sont masqués).

## Paramètres

| Name | Type | Obligatoire | Description |
|------|------|-------------|-------------|
| tenantId | string | Oui |  |
| ids | string | Oui |  |

## Réponse

Renvoie : [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PageUsersInfoResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple d\'utilisation de getUsersInfo'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_78f9';
const ids: string = 'user_10234,user_10235,user_10236';
const usersInfo: PageUsersInfoResponse = await getUsersInfo(tenantId, ids);
// getUsersInfo n'exige que tenantId et ids ; les paramètres optionnels ne s'appliquent pas ici.
[inline-code-end]

---