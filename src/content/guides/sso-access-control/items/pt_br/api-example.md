Aqui vamos percorrer a chamada da API do FastComments para configurar o controle de acesso.

Antes de começar, note que não precisamos criar explicitamente uma estrutura de `Group`. Os grupos são simplesmente identificadores
adicionados a `Users` e `Pages`. Adicionar um grupo a um usuário ou página "cria" o grupo automaticamente.

Primeiro, vamos criar dois usuários, `User A` e `User B`, começando ambos em `Group X`:

[inline-code-attrs-start title = 'Exemplo de cURL para criar o Usuário A'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Exemplo de cURL para criar o Usuário B'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

Agora vamos criar uma `Page`. Vamos chamá-la de nossa `Confidential Page`, e até agora nenhum desses usuários terá acesso a ela, pois ela estará
no grupo `CONFIDENTIAL`:

[inline-code-attrs-start title = 'Exemplo de cURL para POST de Page'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

Os usuários A e B atualmente **NÃO** têm acesso à nova página. No entanto, como estão no mesmo grupo, `GROUP-X`, eles podem se `@mencionar` mutuamente.

Vamos atualizar o `User B` para que ele agora possa acessar a página:

[inline-code-attrs-start title = 'Exemplo de cURL para atualizar o Usuário B'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "groupIds": ["GROUP-X", "CONFIDENTIAL"]
}'
[inline-code-end]

O `User B` agora pertence a ambos os grupos. Nossos usuários ainda podem se `@mencionar` mutuamente, mas apenas o `User B` pode visualizar nossa página confidencial.

Vamos fazer com que o `User B` possa apenas visualizar a página confidencial:

[inline-code-attrs-start title = 'Exemplo de cURL para atualizar o Usuário B'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "groupIds": ["CONFIDENTIAL"]
}'
[inline-code-end]

Agora ele pode visualizar a página confidencial, mas nenhum dos nossos usuários pode se `@mencionar` mutuamente, pois estão em grupos diferentes.

No entanto, qualquer usuário que não faça parte do controle de acesso **poderá acessar nossa página**. Para evitar isso, garanta que nenhum SSO User tenha
seu `groupIds` definido como null. Por exemplo, vamos criar o `User C`, que tem acesso a tudo:

[inline-code-attrs-start title = 'Exemplo de cURL para criar o Usuário C'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

Ao definir `groupIds` como null, dizemos que ele não é limitado pelo controle de acesso.

Agora, vamos criar uma página que todos têm acesso:

[inline-code-attrs-start title = 'Exemplo de cURL para POST de Page'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

Ao definir `accessibleByGroupIds` como null, dizemos que esta `Page` não é controlada pelo controle de acesso, e ambos os usuários podem acessá-la.

Isso conclui nosso passo a passo da API para o Controle de Acesso.