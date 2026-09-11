## Ações e Pesquisas

Ações criam dados no FastComments; pesquisas buscam dados para que uma etapa posterior possa usá‑los. Cada ação chama a API REST do FastComments e consome os mesmos créditos de API que a chamada custaria a partir do seu próprio código: um crédito por chamada, a menos que indicado.

## Criar Comentário

Publica um comentário em uma página.

| Field | Required | Notes |
|-------|----------|-------|
| Page URL ID | Yes | O ID da URL que o widget de comentários usa na página. Comentários são agrupados por ele. |
| Page URL | Yes | A URL completa da página, usada em e‑mails de notificação. |
| Comment | Yes | O corpo do comentário em markdown do FastComments. |
| Commenter Name | Yes | Nomes são únicos por e‑mail, portanto reutilizar um nome com um e‑mail diferente falha. |
| Commenter Email | No | Um usuário é criado para o e‑mail quando ainda não existe. |
| User ID | No | Um ID de usuário SSO existente. Tem precedência sobre o nome e e‑mail. |
| Parent Comment ID | No | Defina para publicar uma resposta. |
| Approved, Verified | No | Ambos padrão são true. Comentários não aprovados permanecem ocultos até serem moderados. |
| Posted At | No | Padrão é agora. |
| Avatar URL, Page Title, Locale | No | Locale padrão é `en_us`. |
| Show Live In Widget | No | Envia o comentário para os visualizadores em tempo real. Custa 2 créditos ao invés de 1. |
| Run Spam Check, Send Emails | No | Desativado por padrão. |

## Criar Página

Cria um registro de página antes que exista qualquer comentário nela, para que possa ser listada e restrita. Recebe o ID da URL, título, URL e, opcionalmente, os IDs de grupos SSO que podem visualizá‑la.

## Criar Usuário SSO

Cria um usuário de login único. Recebe seu próprio ID de usuário, nome de usuário e e‑mail, além de nome de exibição opcional, rótulo de exibição, avatar, site, IDs de grupos e flags de notificação e privacidade. Funções administrativas não podem ser concedidas via Zapier.

## Criar Postagem de Feed

Cria uma postagem em um feed do FastComments a partir de conteúdo HTML. O ID do usuário autor é obrigatório (um ID de usuário FastComments ou SSO); título, tags e uma pré‑visualização de link são opcionais.

## Criar Hashtag

Cria uma hashtag que os comentaristas podem usar, com um URL opcional ao qual ela aponta. As tags são únicas por conta, portanto um Zap que cria uma a cada execução precisa de algo único na tag.

## Sinalizar Comentário

Sinaliza um comentário para revisão de moderador. O ID do usuário que faz a sinalização é obrigatório; o ID do autor retornado por Criar Comentário funciona.

## Pesquisas

| Search | Input | Returns |
|--------|-------|---------|
| Find Comment | Comment ID | O comentário, ou nada. |
| Find SSO User | Email | O usuário SSO, ou nada. |
| Find Page | URL ID | A página, ou nada. |

Uma pesquisa que não encontra nada não falha o Zap. Combine uma pesquisa com uma criação no modo “find or create” do Zapier para criar a página ou usuário quando estiver ausente.