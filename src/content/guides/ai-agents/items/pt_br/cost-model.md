O custo do agente é baseado em **tokens**. Cada chamada de LLM retorna uma contagem de tokens, a plataforma converte isso para centavos de USD usando a tarifa por token do modelo, e os centavos são cobrados contra os orçamentos do agente e do locatário.

### O que é cobrado

- **Todas as chamadas de LLM**, incluindo a chamada que não produz ações de ferramenta ("o agente decidiu não fazer nada"). A inferência é paga mesmo quando nenhuma ação resulta.
- **Chamadas em modo dry-run**. Dry-run é "não agir, mas ainda chamar o LLM" - a chamada ao LLM custa o mesmo. Veja [Modo Dry-Run](#dry-run-mode).
- **Chamadas de replay**. Replays são execuções em modo dry-run contra comentários históricos. Elas consomem tokens. Veja [Test Runs (Replays)](#test-runs-replays).

### O que não é cobrado

- **Gatilhos que nunca produzem uma chamada ao LLM.** Casos descartados antes do LLM (sobre orçamento, rate limit, incompatibilidade de escopo, cobrança inválida, prevenção de loop) custam zero tokens. Veja [Drop Reasons](#drop-reasons).
- **Despacho de ferramentas.** Chamar `pin_comment` ou qualquer outra ferramenta em si não custa tokens - apenas a ida e volta ao LLM custa.
- **`search_memory`.** É somente leitura e não gera sua própria ida e volta ao LLM.

### Custo por execução

Uma única execução do agente pode chamar o LLM várias vezes - cada resultado de chamada de ferramenta é reenviado ao modelo para que ele possa chamar outra ferramenta ou finalizar. Então `tokensUsed` em uma execução é a soma de todas as idas e voltas ao LLM nessa execução.

Os maiores contribuintes para o custo de tokens por execução:

- **Prompts iniciais longos** e **diretrizes da comunidade** - eles entram em todas as execuções.
- **[Context options](#context-options)** - contexto de thread, histórico do usuário, metadados da página. Cada um adiciona tokens.
- **O próprio texto do comentário** - comentários longos custam mais.
- **Múltiplas chamadas de ferramenta em uma execução** - o resultado de cada ferramenta é enviado de volta ao modelo.
- **Leituras de memória** - `search_memory` retorna até 25 registros (limitado a 8000 caracteres de conteúdo total). A maior parte desses bytes entra no próximo prompt.

**Max Tokens Per Trigger** (padrão 20,000) limita o tamanho da **resposta** por chamada ao LLM. Não limita o tamanho da entrada.

### Conversão de tokens para centavos

A plataforma aplica uma única tarifa por pacote do locatário (`flexLLMCostCents` por `flexLLMUnit` tokens). O custo por token é definido no nível do pacote, não por modelo - ambos os modelos disponíveis ([GLM 5.1 and GPT-OSS Turbo](#choosing-a-model)) são cobrados à mesma taxa em um dado pacote. A [Run Detail View](#run-detail-view) mostra o custo por execução na sua moeda assim que uma execução é concluída.

### Onde o custo é registrado

Cada execução registra sua contagem bruta de tokens e o custo por execução. Totais diários e mensais são agregados na [Analytics page](#analytics-page).

### Como interpretar o custo

- **Custo por execução**: [Run Detail View](#run-detail-view) -> campo `Cost`.
- **Agregado diário / mensal**: [Analytics page](#analytics-page) -> gráficos de Uso do Orçamento e Custo Diário.
- **Custo por ação**: também na [Run Detail View](#run-detail-view), útil para ajuste quando o loop de ferramentas de um agente está anormalmente longo.

### Veja também

- [Choosing a Model](#choosing-a-model) - a maior alavanca sobre o custo.
- [Context Options](#context-options) - de onde vem o custo adicional.
- [Budgets Overview](#budgets-overview) - limites rígidos que previnem custos descontrolados.