Informations d'utilisateurs en masse pour un locataire. Étant donné des userIds, retourne les informations d'affichage de User / SSOUser.
Utilisé par le widget de commentaires pour enrichir les utilisateurs qui viennent d'apparaître via un événement de présence.
Pas de contexte de page : la confidentialité est appliquée de façon uniforme (les profils privés sont masqués).

## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| ids | string | Yes |  |

## Réponse

Retourne : [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PageUsersInfoResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple getUsersInfo'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_78f9';
const ids: string = 'user_10234,user_10235,user_10236';
const usersInfo: PageUsersInfoResponse = await getUsersInfo(tenantId, ids);
// getUsersInfo nécessite uniquement tenantId et ids ; les paramètres optionnels ne s'appliquent pas ici.
[inline-code-end]

---