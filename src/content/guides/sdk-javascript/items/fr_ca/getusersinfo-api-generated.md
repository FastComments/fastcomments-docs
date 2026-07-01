Informations d'utilisateur en masse pour un locataire. Étant donné des userIds, renvoie les informations d'affichage provenant de User / SSOUser.  
Utilisé par le widget de commentaires pour enrichir les utilisateurs qui viennent d'apparaître via un événement de présence.  
Pas de contexte de page : la confidentialité est appliquée uniformément (les profils privés sont masqués).

## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|-------------|-------------|
| tenantId | string | Oui |  |
| ids | string | Oui |  |

## Réponse

Renvoie : [`GetUsersInfoResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUsersInfoResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple getUsersInfo'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp-tenant";
const ids: string = "user-1001,user-1002";

const usersInfo: GetUsersInfoResponse = await getUsersInfo(tenantId, ids);

// Les champs optionnels dans la réponse peuvent être indéfinis
const firstUser: PageUserEntry | undefined = usersInfo?.users?.[0];
[inline-code-end]