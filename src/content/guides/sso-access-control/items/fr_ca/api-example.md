---
Nous allons parcourir l'appel à l'API FastComments pour configurer le contrôle d'accès.

Avant de commencer, notez que nous n'avons pas à créer explicitement une structure `Group`. Les groupes sont simplement des identifiants
ajoutés aux `Users` et aux `Pages`. Ajouter un groupe à un utilisateur ou à une page « crée » automatiquement le groupe.

Premièrement, créons deux utilisateurs, `User A` et `User B`; nous allons les placer dans `Group X` :

[inline-code-attrs-start title = 'Exemple cURL : création utilisateur A'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"id": "user-a",
	"username": "User A",
	"email": "usera@example.com",
    "groupIds": ["GROUP-X"]
}'
[inline-code-end]

[inline-code-attrs-start title = 'Exemple cURL : création utilisateur B'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"id": "user-b",
	"username": "User B",
	"email": "userb@example.com",
    "groupIds": ["GROUP-X"]
}'
[inline-code-end]

Créons maintenant une `Page`. Nous l'appellerons `Confidential Page`, et jusqu'à présent aucun de ces utilisateurs n'y aura accès car elle sera dans le groupe `CONFIDENTIAL` :

[inline-code-attrs-start title = 'Exemple cURL POST : page confidentielle'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/pages?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"title": "Confidential Page",
	"url": "https://mysite.com/confidential",
	"urlId": "https://mysite.com/confidential",
	"accessibleByGroupIds": ["CONFIDENTIAL"]
}'
[inline-code-end]

Les utilisateurs A et B n'ont actuellement **PAS** accès à la nouvelle page. Cependant, puisqu'ils appartiennent au même groupe, `GROUP-X`, ils peuvent `@mention` l'un l'autre.

Mettons à jour `User B` pour qu'il puisse maintenant accéder à la page :

[inline-code-attrs-start title = 'Exemple cURL : mise à jour utilisateur B'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "groupIds": ["GROUP-X", "CONFIDENTIAL"]
}'
[inline-code-end]

`User B` appartient maintenant aux deux groupes. Nos utilisateurs peuvent toujours `@mention` l'un l'autre, mais seul `User B` peut voir notre page confidentielle.

Faisons en sorte que `User B` puisse uniquement voir la page confidentielle :

[inline-code-attrs-start title = 'Exemple cURL : mise à jour utilisateur B'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "groupIds": ["CONFIDENTIAL"]
}'
[inline-code-end]

Ils peuvent désormais voir la page confidentielle, mais aucun de nos utilisateurs ne peut se `@mention`ner, car ils sont dans des groupes différents.

Cependant, tout utilisateur qui n'est pas soumis au contrôle d'accès **pourra accéder à notre page**. Pour l'empêcher, assurez-vous qu'aucun utilisateur SSO n'ait ses `groupIds` définis à null. Par exemple, créons `User C`, qui a accès à tout :

[inline-code-attrs-start title = 'Exemple cURL : création utilisateur C'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"id": "user-c",
	"username": "User C",
	"email": "userc@example.com",
    "groupIds": null
}'
[inline-code-end]

En définissant `groupIds` sur null, nous indiquons qu'ils ne sont pas limités par le contrôle d'accès.

Maintenant, créons une page à laquelle tout le monde a accès :

[inline-code-attrs-start title = 'Exemple cURL POST : page publique'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/pages?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"title": "Public Page",
	"url": "https://mysite.com/public",
	"urlId": "https://mysite.com/public",
	"accessibleByGroupIds": null
}'
[inline-code-end]

En définissant `accessibleByGroupIds` sur null, nous indiquons que cette `Page` n'est pas soumise au contrôle d'accès, et que les deux utilisateurs peuvent y accéder.

Ceci conclut notre présentation de l'API pour le contrôle d'accès.

---