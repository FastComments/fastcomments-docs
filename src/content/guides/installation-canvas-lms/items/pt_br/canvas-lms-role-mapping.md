As funções do Canvas são mapeadas automaticamente para as funções do FastComments durante o lançamento LTI. Nenhuma configuração manual é necessária.

#### Mapeamento de Funções

| Canvas Role | FastComments Role | Permissions |
|---|---|---|
| **Administrator** | Admin | Acesso total à conta, gerenciar todos os comentários e configurações |
| **Instructor** | Moderator | Editar e excluir comentários, fixar tópicos, gerenciar discussões |
| **Learner** | Commenter | Publicar comentários, responder, votar e usar menções |

#### Como Funciona

Quando um usuário inicia o FastComments a partir do Canvas, o protocolo LTI 1.3 inclui sua função no Canvas. O FastComments lê essa função e atribui automaticamente as permissões apropriadas.

Se um usuário possuir múltiplas funções (por exemplo, um Instructor que também é Admin), a função com maior privilégio será usada.

---