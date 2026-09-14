---
[Val Town](https://val.town) executa TypeScript no Deno, então um val é um servidor real. Isso o torna uma boa escolha para o FastComments: o widget é uma tag script na página, e qualquer coisa que precise de um segredo, como Secure SSO ou verificação de webhook, pode ser executada no lado do servidor no mesmo val.

Este guia cobre a adição do widget de comentários a um val HTTP, a exibição de contagens de comentários em uma página de índice, o login de usuários com a conta Val Town que eles já possuem e o recebimento de webhooks de comentários.

Você não precisa de uma conta para experimentá-lo. Os exemplos usam `tenantId: "demo"`, um sandbox compartilhado, e o Passo 2 cobre a troca para o seu próprio.
---