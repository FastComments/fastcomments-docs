Aquí recorreremos el proceso de llamar a la API de FastComments para configurar el control de acceso.

Antes de comenzar, ten en cuenta que no tenemos que crear explícitamente una estructura `Group`. Los grupos son simplemente identificadores
añadidos a `Users` y `Pages`. Añadir un grupo a un usuario o página "crea" automáticamente el grupo.

Primero, creemos dos usuarios, `User A` y `User B`, los empezaremos en `Group X`:

[inline-code-attrs-start title = 'Ejemplo de cURL para crear el Usuario A'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Ejemplo de cURL para crear el Usuario B'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

Ahora creemos una `Page`. La llamaremos nuestra `Confidential Page`, y hasta ahora ninguno de estos usuarios tendrá acceso a ella, ya que estará
en el grupo `CONFIDENTIAL`:

[inline-code-attrs-start title = 'Ejemplo de cURL POST para la página'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

Los usuarios A y B actualmente **NO** tienen acceso a la nueva página. Sin embargo, como están en el mismo grupo, `GROUP-X`, pueden `@mention`arse entre sí.

Actualicemos `User B` para que ahora pueda acceder a la página:

[inline-code-attrs-start title = 'Ejemplo de cURL para actualizar el Usuario B'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "groupIds": ["GROUP-X", "CONFIDENTIAL"]
}'
[inline-code-end]

`User B` ahora pertenece a ambos grupos. Nuestros usuarios todavía pueden `@mention`arse entre sí, pero solo `User B` puede ver nuestra página confidencial.

Hagamos que `User B` solo pueda ver la página confidencial:

[inline-code-attrs-start title = 'Ejemplo de cURL para actualizar el Usuario B'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "groupIds": ["CONFIDENTIAL"]
}'
[inline-code-end]

Ahora pueden ver la página confidencial, pero ninguno de nuestros usuarios puede `@mention`arse entre sí, ya que están en grupos diferentes.

Sin embargo, cualquier usuario que no forme parte del control de acceso **podrá acceder a nuestra página**. Para evitar esto, asegúrate de que ningún SSO User tenga
sus `groupIds` establecidos en null. Por ejemplo, creemos `User C`, que tiene acceso a todo:

[inline-code-attrs-start title = 'Ejemplo de cURL para crear el Usuario C'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

Al establecer `groupIds` en null, indicamos que no está limitado por el control de acceso.

Ahora, creemos una página a la que todos tengan acceso:

[inline-code-attrs-start title = 'Ejemplo de cURL POST para la página'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

Al establecer `accessibleByGroupIds` en null, indicamos que esta `Page` no está controlada por el control de acceso, y ambos usuarios pueden acceder a ella.

Con esto completamos nuestro recorrido por la API para el Control de Acceso.