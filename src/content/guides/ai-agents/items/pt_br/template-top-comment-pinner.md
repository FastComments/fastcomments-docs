**ID do Template:** `top_comment_pinner`

O Top Comment Pinner monitora comentários de nível superior que ultrapassam um limite de votos e os fixa - substituindo o que estava fixado anteriormente na mesma thread.

O prompt embutido instrui o agente a ignorar respostas (a fixação funciona em threads, então fixar uma resposta raramente é útil) e a filtrar conteúdo promocional (para que o agente não impulsione link-spam popular).

### Gatilhos

- **Um comentário ultrapassa um limite de votos** (`COMMENT_VOTE_THRESHOLD`, limite de votos padrão: 10).

O gatilho é acionado quando os votos líquidos do comentário (`up - down`) atingem o limite configurado. Ajuste o número no formulário de edição com base na atividade das suas threads - 10 é um padrão razoável para sites moderadamente ativos.

### Ferramentas permitidas

- [`pin_comment`](#tools-overview)
- [`unpin_comment`](#tools-overview)

Fixar não é destrutivo - pode ser revertido instantaneamente - então este template geralmente roda sem aprovações.

### Ajustes recomendados antes de entrar em produção

- **Marque "Include parent comment and prior replies in the same thread"** em [Context Options](#context-options). Sem o contexto da thread, o agente não consegue identificar de forma confiável se já existe um comentário fixado para desafixar.
- **Ajuste o limite de votos** para o seu site. Em threads movimentadas, 10 acontece com muita frequência; em threads tranquilas, 10 pode nunca acontecer.
- **Considere limitar por URL** se você quiser comentários fixados apenas em certas seções do seu site - threads de notícias, por exemplo, mas não threads de anúncios.

### Observação sobre fixação duplicada

O prompt do agente instrui a desafixar primeiro antes de fixar, mas se o modelo perder essa etapa, a plataforma em si não impõe uma regra de um-fixado-por-thread (você pode ter vários). Se a fixação duplicada for um problema no seu site, exija aprovação para `pin_comment` e revise cada um - ou escreva um prompt mais restritivo.