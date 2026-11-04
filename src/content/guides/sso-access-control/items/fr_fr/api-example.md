Nous allons voir ici comment appeler l'API FastComments pour configurer le contrôle d'accès.

Avant de commencer, notez que nous n'avons pas besoin de créer explicitement une structure `Group`. Les groupes sont simplement des identifiants
ajoutés aux `Users` et aux `Pages`. L'ajout d'un groupe à un utilisateur ou à une page « crée » automatiquement le groupe.

Commençons par créer deux utilisateurs, `User A` et `User B`, que nous placerons initialement dans le `Group X` :

[inline-code-attrs-start title = 'Exemple cURL de création de l\'utilisateur A'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Exemple cURL de création de l\'utilisateur B'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

Créons maintenant une `Page`. Nous l'appellerons notre `Confidential Page`, et pour l'instant aucun de ces utilisateurs n'y aura accès, car elle sera
dans le groupe `CONFIDENTIAL` :

[inline-code-attrs-start title = 'Exemple cURL de POST de Page'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

Les utilisateurs A et B n'ont actuellement **PAS** accès à la nouvelle page. Cependant, comme ils sont dans le même groupe, `GROUP-X`, ils peuvent se `@mention` mutuellement.

Mettons à jour `User B` pour qu'il puisse désormais accéder à la page :

[inline-code-attrs-start title = 'Exemple cURL de mise à jour de l\'utilisateur B'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "groupIds": ["GROUP-X", "CONFIDENTIAL"]
}'
[inline-code-end]

`User B` appartient maintenant aux deux groupes. Nos utilisateurs peuvent toujours se `@mention` mutuellement, mais seul `User B` peut consulter notre page confidentielle.

Faisons en sorte que `User B` ne puisse consulter que la page confidentielle :

[inline-code-attrs-start title = 'Exemple cURL de mise à jour de l\'utilisateur B'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "groupIds": ["CONFIDENTIAL"]
}'
[inline-code-end]

Il peut désormais consulter la page confidentielle, mais aucun de nos deux utilisateurs ne peut se `@mention` mutuellement, car ils sont dans des groupes différents.

Cependant, tout utilisateur ne faisant pas partie du contrôle d'accès **pourra accéder à notre page**. Pour éviter cela, assurez-vous qu'aucun utilisateur SSO
n'ait ses `groupIds` définis à null. Par exemple, créons `User C`, qui a accès à tout :

[inline-code-attrs-start title = 'Exemple cURL de création de l\'utilisateur C'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

En définissant `groupIds` à null, nous indiquons qu'il n'est pas soumis au contrôle d'accès.

Créons maintenant une page à laquelle tout le monde a accès :

[inline-code-attrs-start title = 'Exemple cURL de POST de Page'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

En définissant `accessibleByGroupIds` à null, nous indiquons que cette `Page` n'est pas soumise au contrôle d'accès, et les deux utilisateurs peuvent y accéder.

Ceci conclut notre présentation de l'API pour le contrôle d'accès.