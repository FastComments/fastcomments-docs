**ID do Template:** `thread_summarizer`

O Thread Summarizer publica um resumo neutro, em um único parágrafo, ao final de um thread longo. Ele usa um adiamento de 30 minutos para que o thread possa se estabilizar antes que o agente o analise.

### Prompt inicial incorporado

[inline-code-attrs-start title = 'Prompt Inicial do Template Thread Summarizer'; type='text' inline-code-attrs-end]
[inline-code-start]
You post neutral thread summaries. Do not summarize threads that have fewer than 5 comments. For longer threads, summarize the main positions, disagreements, and open questions in one short paragraph. Do not take sides and do not editorialize. After posting the summary, pin it. If a prior summary by you is already pinned on this thread, unpin it before pinning the new one.
[inline-code-end]

A instrução "do not editorialize" é essencial. Sem ela, o modelo tende a enquadrar com "in my view", o que soa mal sob o nome de exibição da sua conta.

### Gatilhos

- **Novo comentário publicado** (`COMMENT_ADD`).
- **Atraso do gatilho**: 30 minutos (1800 segundos). Veja [Gatilhos Diferidos](#trigger-deferred-delay).

O atraso de 30 minutos significa que o agente executa uma vez, meia hora após a chegada de um comentário, contra o estado do thread naquele momento. Não é "resumir a cada resposta" — a fila de gatilhos diferidos consolida múltiplos eventos de novo comentário no mesmo thread, mas não os desduplica através de janelas separadas. Você provavelmente vai querer **adicionar uma regra personalizada no seu prompt** como "não postar um novo resumo se o agente já resumiu este thread nas últimas 24 horas" e confiar no contexto mais as [ferramentas de memória](#tools-overview) do agente para aplicá-la.

### Ferramentas permitidas

- [`write_comment`](#tools-overview) - publica o resumo em si.
- [`pin_comment`](#tools-overview) - fixa o resumo para que os leitores o vejam no topo do thread.
- [`unpin_comment`](#tools-overview) - desfixa um resumo anterior feito pelo mesmo agente antes de fixar o novo.

O resumidor não pode moderar ou interagir com usuários.

### Fixando o resumo

O agente publica um novo comentário com `write_comment`, então chama `pin_comment` com o ID do comentário retornado. Em execuções subsequentes contra o mesmo thread, o prompt instrui-o a chamar `unpin_comment` no seu resumo anterior primeiro — a plataforma em si não aplica a regra de comentário único fixado por thread, então deixar o resumo anterior fixado resultará em dois resumos fixados lado a lado. Marque "Include parent comment and prior replies in the same thread" em [Opções de Contexto](#context-options) para que o agente possa ver o resumo anterior fixado.

### Adições recomendadas antes de entrar em produção

- **Marque "Include parent comment and prior replies in the same thread"** em [Opções de Contexto](#context-options). Um resumidor sem contexto do thread é inútil.
- **Ajuste a regra de tamanho mínimo do thread.** "Fewer than 5 comments" é o padrão do prompt, mas em comunidades movimentadas 10–20 é mais apropriado. Edite o prompt diretamente.
- **Restringir a padrões de URL específicos** se você quiser resumos apenas em páginas long-form, não em anúncios ou páginas de produto. Veja [Escopo: Filtros de URL e Localidade](#scope-url-locale).
- **Monitore os custos.** A sumarização é o template que mais consome tokens porque lê o thread inteiro a cada execução. Defina um [orçamento diário](#budgets-overview) apertado antes de ativar.

### Evitando resumos repetidos

O agente tem acesso a [`save_memory`](#tools-overview) e [`search_memory`](#tools-overview) — você pode estender o prompt para instruí-lo a registrar notas como "resumido {thread urlId}" e verificá-las antes de publicar novamente. A memória é compartilhada entre todos os agentes no seu tenant.