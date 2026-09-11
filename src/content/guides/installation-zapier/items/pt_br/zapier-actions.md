## Ações e Pesquisas

Ações criam dados no FastComments; pesquisas buscam dados para que uma etapa posterior possa usá‑los. Cada ação chama a API REST do FastComments e consome os mesmos créditos de API que a chamada custaria no seu próprio código: um crédito por chamada, a menos que indicado.

## Criar Comentário

Publica um comentário em uma página.

| Campo | Obrigatório | Observações |
|-------|-------------|-------------|
| ID da URL da Página | Sim | O ID da URL que o widget de comentários usa na página. Os comentários são agrupados por ele. |
| URL da Página | Sim | A URL completa da página, usada em e‑mails de notificação. |
| Comentário | Sim | O corpo do comentário em markdown do FastComments. |
| Nome do Comentador | Sim | Os nomes são únicos por e‑mail, portanto reutilizar um nome com um e‑mail diferente falha. |
| E‑mail do Comentador | Não | Um usuário é criado para o e‑mail quando ainda não existe. |
| ID do Usuário | Não | Um ID de usuário SSO existente. Tem precedência sobre o nome e o e‑mail. |
| ID do Comentário Pai | Não | Defina para publicar uma resposta. |
| Aprovado, Verificado | Não | Ambos padrão como verdadeiro. Comentários não aprovados permanecem ocultos até serem moderados. |
| Publicado Em | Não | Padrão para agora. |
| URL do Avatar, Título da Página, Localidade | Não | Localidade padrão é `en_us`. |
| Mostrar ao Vivo no Widget | Não | Envia o comentário para os visualizadores em tempo real. Custa 2 créditos ao invés de 1. |
| Executar Verificação de Spam, Enviar E‑mails | Não | Desativado por padrão. |

## Criar ou Atualizar Página

Cria um registro de página antes que exista qualquer comentário nela, permitindo que seja listada e restrita. Recebe o ID da URL, título, URL e, opcionalmente, os IDs de grupos SSO que podem visualizá‑la. Se uma página com esse ID de URL já existir, ela é atualizada com os campos fornecidos, permitindo que um Zap seja executado repetidamente para a mesma página.

## Criar ou Atualizar Usuário SSO

Cria um usuário de login único (single sign‑on). Recebe seu próprio ID de usuário, nome de usuário e e‑mail, além de nome de exibição opcional, rótulo de exibição, avatar, site, IDs de grupos e sinalizadores de notificação e privacidade. Se um usuário com esse ID já existir, ele é atualizado em vez de criado. Funções administrativas não podem ser concedidas via Zapier.

## Criar Postagem no Feed

Cria uma postagem em um feed do FastComments a partir de conteúdo HTML. O ID do usuário autor é obrigatório (um ID de usuário FastComments ou SSO); título, tags e uma pré‑visualização de link são opcionais.

## Criar ou Atualizar Tag de Hash

Cria uma tag de hash que os comentadores podem usar, com uma URL opcional para a qual ela aponta. Se a tag já existir, ela é atualizada em vez de criada.

## Sinalizar Comentário

Sinaliza um comentário para revisão de moderador. O ID do usuário que faz a sinalização é obrigatório; o ID do autor retornado por Criar Comentário funciona.

## Pesquisas

| Pesquisa | Entrada | Retorna |
|----------|---------|---------|
| Encontrar Comentário | ID do Comentário | O comentário, ou nada. |
| Encontrar Usuário SSO | E‑mail | O usuário SSO, ou nada. |
| Encontrar Página | ID da URL | A página, ou nada. |

Uma pesquisa que não encontra nada não falha o Zap. Encontrar Usuário SSO e Encontrar Página oferecem a opção "criar se não existir" do Zapier, que executa a criação correspondente quando nada é encontrado.