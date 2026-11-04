Enquetes também podem ser gerenciadas a partir do seu próprio código com a API REST, por exemplo, para adicionar uma enquete a um comentário a partir do seu backend, encerrar uma enquete ou exportar os votos:

- [Estrutura da Enquete](/guide-api.html#poll-structure) descreve o objeto de enquete e seus limites.
- [GET](/guide-api.html#poll-get), [PUT](/guide-api.html#poll-put), [PATCH](/guide-api.html#poll-patch) e [DELETE](/guide-api.html#poll-delete) `/api/v1/polls/:commentId` leem, substituem, editam e removem a enquete de um comentário.
- [Estrutura de Voto](/guide-api.html#poll-vote-structure) descreve um voto individual.
- [GET](/guide-api.html#poll-votes-get), [POST](/guide-api.html#poll-vote-post) e [DELETE](/guide-api.html#poll-vote-delete) `/api/v1/poll-votes` listam, registram e removem votos.

Uma enquete também pode ser criada junto com seu comentário passando `poll` para `POST /api/v1/comments`.

A API segue as mesmas configurações do widget: as enquetes devem estar habilitadas, e votos sem um usuário autenticado são rejeitados, a menos que votos anônimos em enquetes sejam permitidos. Votos individuais em enquetes anônimas não podem ser lidos através da API.