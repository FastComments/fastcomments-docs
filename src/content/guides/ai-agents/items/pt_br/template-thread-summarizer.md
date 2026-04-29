---
**Template ID:** `thread_summarizer`

O Thread Summarizer publica um resumo neutro, em um único parágrafo, ao final de um tópico longo. Ele usa um adiamento de 30 minutos para que o tópico se estabilize antes que o agente o analise.

O prompt embutido instrui o agente a não adotar um tom editorial — isso é essencial. Sem essa instrução, o modelo tende a usar formulações como "na minha opinião" que soam mal ao serem exibidas com o nome da sua conta.

### Gatilhos

- **New comment posted** (`COMMENT_ADD`).
- **Trigger delay**: 30 minutes (1800 seconds). See [Gatilhos Diferidos](#trigger-deferred-delay).

O atraso de 30 minutos significa que o agente é executado uma vez, meia hora após a chegada de um comentário, contra o estado do tópico naquele momento. Não é "resumir a cada resposta" — a fila de gatilhos diferidos agrupa múltiplos eventos de novo comentário no mesmo tópico, mas não os desduplicará em janelas de tempo separadas. Você provavelmente vai querer **adicionar uma regra personalizada no seu prompt** como "não publicar um novo resumo se o agente já resumiu este tópico nas últimas 24 horas" e confiar no contexto mais as [ferramentas de memória](#tools-overview) do agente para fazer cumprir isso.

### Ferramentas permitidas

- [`write_comment`](#tools-overview) - publica o próprio resumo.
- [`pin_comment`](#tools-overview) - fixa o resumo para que os leitores o vejam no topo do tópico.
- [`unpin_comment`](#tools-overview) - desafixa um resumo anterior feito pelo mesmo agente antes de fixar o novo.

O sumarizador não pode moderar nem interagir com usuários.

### Fixando o resumo

O agente publica um novo comentário com `write_comment`, depois chama `pin_comment` com o ID do comentário retornado. Em execuções subsequentes contra o mesmo tópico, o prompt instrui que ele chame `unpin_comment` no seu resumo anterior primeiro — a própria plataforma **não** aplica uma regra de um único comentário fixado por tópico, então deixar o resumo anterior fixado resultará em dois resumos fixados lado a lado. Marque "Incluir comentário pai e respostas anteriores no mesmo tópico" em [Opções de Contexto](#context-options) para que o agente possa ver o resumo fixado anterior.

### Recomendações antes de entrar em produção

- **Marque "Incluir comentário pai e respostas anteriores no mesmo tópico"** em [Opções de Contexto](#context-options). Um sumarizador sem contexto do tópico é inútil.
- **Ajuste a regra de tamanho mínimo do tópico.** "Fewer than 5 comments" é o padrão do prompt, mas em comunidades movimentadas 10–20 é mais apropriado. Edite o prompt diretamente.
- **Restrinja a padrões de URL específicos** se você quiser resumos apenas em páginas de formato longo, não em anúncios ou páginas de produto. Veja [Escopo: filtros de URL e de localidade](#scope-url-locale).
- **Monitore custos.** A sumarização é o template que mais consome tokens porque lê o tópico inteiro a cada execução. Defina um [orçamento diário](#budgets-overview) rígido antes de ativar.

### Evitando resumos repetidos

O agente tem acesso a [`save_memory`](#tools-overview) e [`search_memory`](#tools-overview) — você pode estender o prompt para instruí-lo a registrar notas do tipo "summarized {thread urlId}" e verificá-las antes de publicar novamente. A memória é compartilhada entre todos os agentes do seu tenant.

---