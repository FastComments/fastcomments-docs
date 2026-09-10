## Gatilhos

Os gatilhos iniciam um Zap quando algo acontece no FastComments. Todos os três são instantâneos: o FastComments entrega o evento ao Zapier através de um webhook no momento em que ocorre. Nada faz polling na sua conta e nenhum crédito de API é gasto aguardando.

| Gatilho | Dispara quando |
|---------|----------------|
| Novo Comentário | Um comentário é publicado. Por padrão, apenas comentários aprovados e que não são spam disparam. |
| Comentário Atualizado | Um comentário é editado, aprovado, votado, fixado, bloqueado ou alterado de outra forma. |
| Comentário Excluído | Um comentário é excluído. |

Cada gatilho retorna o comentário completo: id, URL da página e ID da URL, nome e e‑mail do comentarista, o texto do comentário como markdown e como HTML, contagens de votos, sinalizadores de aprovação e spam, o locale, o domínio e quaisquer menções. Os campos correspondem à carga útil do webhook documentada em Webhooks, Estruturas de Dados.

## Opções

**Domínio.** Cada gatilho tem um filtro de domínio opcional, listando os domínios configurados na sua conta. Deixe em branco para receber eventos de todos os domínios.

**Incluir Comentários Não Aprovados e Spam.** Apenas no gatilho Novo Comentário. Comentários que ficam retidos para moderação ou marcados como spam são ignorados por padrão. Quando tal comentário é aprovado posteriormente, o gatilho Comentário Atualizado dispara para ele, de modo que um Zap que deve reagir a todo comentário que se torna visível usa Comentário Atualizado com um filtro no campo aprovado.

## Como a entrega funciona

Ativar um Zap cria uma assinatura de webhook na sua conta, visível na página Webhooks com a origem **API**. Desativar o Zap a remove. Os limites próprios do Zapier se aplicam ao número de eventos que ele aceita por minuto; o FastComments tenta novamente uma entrega que falha, com atraso crescente, e desabilita uma assinatura que continua falhando por seis dias. Uma assinatura desabilitada pode ser reativada a partir da página Webhooks, ou simplesmente desligue e ligue o Zap novamente para criar uma nova.

Uma conta pode conter até 50 assinaturas de API. Cada Zap que usa um gatilho FastComments consome uma.