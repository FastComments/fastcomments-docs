## Ações e Pesquisas

Ações criam dados no FastComments; pesquisas buscam dados para que uma etapa posterior possa usá-los. Cada ação chama a API REST do FastComments e consome os mesmos créditos de API que a chamada custaria no seu próprio código: um crédito por chamada, a menos que indicado.

## Criar Comentário

Publica um comentário em uma página.

| Campo | Obrigatório | Observações |
|-------|--------------|-------------|
| ID da URL da página | Sim | O ID da URL que o widget de comentários usa na página. Comentários são agrupados por ele. |
| URL da página | Sim | A URL completa da página, usada em e‑mails de notificação. |
| Comentário | Sim | O corpo do comentário em markdown do FastComments. |
| Nome do Comentador | Sim | Nomes são únicos por e‑mail, portanto reutilizar um nome com um e‑mail diferente falha. |
| E‑mail do Comentador | Não | Um usuário é criado para o e‑mail quando ainda não existe. |
| ID do Usuário | Não | Um ID de usuário SSO existente. Tem precedência sobre o nome e o e‑mail. |
| ID do Comentário Pai | Não | Defina para publicar uma resposta. |
| Aprovado, Verificado | Não | Ambos padrão como verdadeiro. Comentários não aprovados permanecem ocultos até serem moderados. |
| Publicado Em | Não | Padrão para agora. |
| URL do Avatar, Título da Página, Local | Não | Local padrão é `en_us`. |
| Mostrar ao Vivo no Widget | Não | Envia o comentário para os visualizadores em tempo real. Custa 2 créditos ao invés de 1. |
| Executar Verificação de Spam, Enviar E‑mails | Não | Desativado por padrão. |

## Criar Página

Cria um registro de página antes que exista qualquer comentário nela, para que possa ser listada e restrita. Recebe o ID da URL, título, URL e, opcionalmente, os IDs de grupos SSO que podem visualizá‑la.

## Criar Usuário SSO

Cria um usuário de login único (single sign‑on). Recebe seu próprio ID de usuário, nome de usuário e e‑mail, além de nome de exibição opcional, rótulo de exibição, avatar, site, IDs de grupos e sinalizadores de notificação e privacidade. Funções administrativas não podem ser concedidas via Zapier.

## Criar Postagem no Feed

Cria uma postagem em um feed do FastComments a partir de conteúdo HTML, com título opcional, autor, tags e uma pré‑visualização de link.

## Criar Tag de Hash

Cria uma tag de hash que os comentadores podem usar, com uma URL opcional para a qual ela aponta.

## Sinalizar Comentário

Sinaliza um comentário para revisão de moderador. Forneça o ID do usuário que está sinalizando, ou deixe em branco para sinalizar como a integração Zapier.

## Pesquisas

| Pesquisa | Entrada | Retorna |
|----------|---------|---------|
| Encontrar Comentário | ID do Comentário | O comentário, ou nada. |
| Encontrar Usuário SSO | E‑mail | O usuário SSO, ou nada. |
| Encontrar Página | ID da URL | A página, ou nada. |

Uma pesquisa que não encontra nada não falha o Zap. Combine uma pesquisa com uma criação no modo “encontrar ou criar” do Zapier para criar a página ou o usuário quando estiver ausente.