Informations groupées sur les utilisateurs pour un locataire. Étant donné des userIds, renvoie les informations d'affichage provenant de User / SSOUser.
Utilisé par le widget de commentaires pour enrichir les utilisateurs qui viennent d'apparaître via un événement de présence.
Pas de contexte de page : la confidentialité est appliquée uniformément (les profils privés sont masqués).

## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| ids | string | Oui |  |

## Réponse

Renvoie : [`GetUsersInfo200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUsersInfo200Response.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple getUsersInfo'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-tenant-007';
const userIdsList: string[] = ['user_12a', 'user_34b', 'user_56c'];
const separator: string | undefined = undefined; // optionnel; si indéfini, la valeur par défaut est la virgule
const ids: string = userIdsList.join(separator ?? ',');
const usersInfo: GetUsersInfo200Response = await getUsersInfo(tenantId, ids);
[inline-code-end]

---