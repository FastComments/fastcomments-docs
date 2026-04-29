---
**ID do modelo:** `top_comment_pinner`

O Top Comment Pinner observa comentários de nível superior que ultrapassam um limite de votos e os fixa - substituindo o que quer que estivesse fixado anteriormente no mesmo tópico.

O prompt embutido instrui o agente a pular respostas (fixar funciona em tópicos, então fixar uma resposta raramente é útil) e a filtrar conteúdo promocional (para que o agente não aumente a visibilidade de spam de links populares).

### Triggers

- **Um comentário ultrapassa um limite de votos** (`COMMENT_VOTE_THRESHOLD`, limite de votos padrão: 10).

O gatilho dispara quando os votos líquidos do comentário (`up - down`) atingem o limite configurado. Ajuste o número no formulário de edição com base em quão ativos seus tópicos são - 10 é um padrão sensato para sites moderadamente ativos.

### Allowed tools

- [`pin_comment`](#tools-overview)
- [`unpin_comment`](#tools-overview)

Fixar é não-destrutivo - pode ser revertido instantaneamente - então este template normalmente roda sem aprovações.

### Recommended additions before going live

- **Marque "Incluir comentário pai e respostas anteriores no mesmo tópico"** em [Opções de Contexto](#context-options). Sem o contexto do tópico, o agente não consegue dizer com confiabilidade se já existe um comentário fixado para desfixar.
- **Ajuste o limite de votos** para o seu site. Em tópicos movimentados, 10 acontece com muita frequência; em tópicos quietos, 10 pode nunca acontecer.
- **Considere restringir por URL** se você quiser comentários fixados apenas em certas seções do seu site - por exemplo, tópicos de notícias, mas não tópicos de anúncios.

### Nota sobre pinagens duplicadas

O prompt do agente instrui para desfixar primeiro antes de fixar, mas se o modelo perder essa etapa, a própria plataforma não aplica uma regra de um fixado por tópico (você pode ter múltiplos). Se pinagens duplicadas forem um problema no seu site, coloque `pin_comment` sob aprovação e revise cada uma - ou escreva um prompt mais rigoroso.

---