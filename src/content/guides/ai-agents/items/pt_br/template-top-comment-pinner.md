**ID do modelo:** `top_comment_pinner`

O Top Comment Pinner monitora comentários de nível superior que ultrapassam um limite de votos e os fixa - substituindo o que estava fixado anteriormente no mesmo tópico.

### Prompt inicial integrado

[inline-code-attrs-start title = 'Prompt inicial do modelo Top Comment Pinner'; type='text' inline-code-attrs-end]
[inline-code-start]
Você fixa os melhores comentários de nível superior em um tópico. Quando um comentário alcança o limite de votos, fixe-o se o conteúdo for substancial e não promocional. Remova a fixação de qualquer comentário previamente fixado no mesmo tópico primeiro. Não fixe respostas, apenas comentários de nível superior.
[inline-code-end]

A instrução "não fixe respostas" é importante: a fixação funciona por tópico, então fixar uma resposta raramente é útil. O filtro "não promocional" impede que o agente impulsione um comentário popular de spam de links.

### Gatilhos

- **Um comentário ultrapassa um limite de votos** (`COMMENT_VOTE_THRESHOLD`, limite de votos padrão: 10).

O gatilho é acionado quando os votos líquidos do comentário (`up - down`) alcançam o limite configurado. Ajuste o número no formulário de edição com base em quão ativos são seus tópicos - 10 é um padrão sensato para sites moderadamente ativos.

### Ferramentas permitidas

- [`pin_comment`](#tools-overview)
- [`unpin_comment`](#tools-overview)

Fixar não é destrutivo - pode ser revertido instantaneamente - então este modelo costuma ser executado sem aprovações.

### Adições recomendadas antes de entrar em produção

- **Marque "Incluir o comentário pai e respostas anteriores no mesmo tópico"** em [Opções de contexto](#context-options). Sem o contexto do tópico o agente não pode dizer de forma confiável se já existe um comentário fixado para desfixar.
- **Ajuste o limite de votos** para o seu site. Em tópicos movimentados, 10 acontece com muita frequência; em tópicos tranquilos, 10 pode nunca acontecer.
- **Considere delimitar por URL** se você quiser comentários fixados apenas em determinadas seções do seu site - por exemplo, tópicos de notícias, mas não tópicos de anúncios.

### Nota sobre fixações duplicadas

O prompt do agente instrui-o a desfixar primeiro antes de fixar, mas se o modelo perder essa etapa a própria plataforma não impõe uma regra de um fixado por tópico (você pode ter múltiplos). Se fixações duplicadas forem um problema no seu site, coloque `pin_comment` atrás de aprovação e revise cada uma - ou escreva um prompt mais restrito.