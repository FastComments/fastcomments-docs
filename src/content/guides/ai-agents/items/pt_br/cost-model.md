O custo do agente é **baseado em tokens**. Cada chamada de LLM retorna uma contagem de tokens, a plataforma converte isso em centavos de USD usando a taxa por token do modelo, e os centavos são cobrados dos orçamentos do agente e do tenant.

### O que é cobrado

- **Todas as chamadas de LLM**, incluindo a chamada que resulta em zero ações de ferramenta ("o agente decidiu não fazer nada"). A inferência é paga mesmo quando nenhuma ação resulta.
- **Chamadas em modo dry-run**. O dry-run é "não agir, mas ainda chamar o LLM" - a chamada de LLM custa o mesmo. Veja [Modo Dry-Run](#dry-run-mode).
- **Chamadas de replay**. Replays são execuções em modo dry-run contra comentários históricos. Elas custam tokens. Veja [Execuções de Teste (Replays)](#test-runs-replays).

### O que não é cobrado

- **Triggers que nunca produzem uma chamada de LLM.** Casos descartados antes do LLM (acima do orçamento, limite de taxa atingido, incompatibilidade de escopo, cobrança inválida, prevenção de loop) custam zero tokens. Veja [Motivos de Descarte](#drop-reasons).
- **Despacho de ferramentas.** Chamar `pin_comment` ou qualquer outra ferramenta não custa tokens por si só - apenas a ida e volta do LLM custa.
- **`search_memory`.** É somente leitura e não produz sua própria ida e volta ao LLM.

### Custo por execução

Uma única execução do agente pode chamar o LLM várias vezes - o resultado de cada chamada de ferramenta é realimentado no modelo para que ele possa chamar outra ferramenta ou finalizar. Portanto, `tokensUsed` em uma execução é a soma de todas as idas e voltas do LLM nessa execução.

Os maiores contribuintes para o custo de tokens por execução:

- **[Prompts iniciais](#personality-prompt) longos e [diretrizes da comunidade](#community-guidelines)** - eles entram em todas as execuções.
- **[Opções de contexto](#context-options)** - contexto do thread, histórico do usuário, metadados da página. Cada um adiciona tokens.
- **O próprio texto do comentário** - comentários longos custam mais.
- **Múltiplas chamadas de ferramenta em uma execução** - a mensagem de resultado de cada ferramenta é enviada de volta ao modelo.
- **Leituras de memória** - `search_memory` retorna até 25 registros (limitados a 8000 caracteres de conteúdo total). A maior parte desses bytes vai para o próximo prompt.

**Max Tokens Per Trigger** (padrão 20.000) limita o tamanho da **resposta** por chamada de LLM. Ele não limita o tamanho da entrada.

### Conversão de tokens para centavos

A plataforma aplica uma única taxa por pacote de tenant (`flexLLMCostCents` por `flexLLMUnit` tokens). O custo por token é no nível do pacote, não por modelo - ambos os modelos disponíveis ([GLM 5.1 e GPT-OSS Turbo](#choosing-a-model)) são cobrados à mesma taxa em um determinado pacote. A [Visualização de Detalhes da Execução](#run-detail-view) mostra o custo por execução na sua moeda assim que a execução é concluída.

### Onde o custo é registrado

Cada execução registra sua contagem bruta de tokens e o custo por execução. Os totais diários e mensais são consolidados na [página de Analytics](#analytics-page).

### Como ler o custo

- **Custo por execução**: [Visualização de Detalhes da Execução](#run-detail-view) -> campo `Cost`.
- **Agregado diário / mensal**: [página de Analytics](#analytics-page) -> gráficos de uso do orçamento e custo diário.
- **Custo por ação**: também na Visualização de Detalhes da Execução, útil para ajustes quando o loop de ferramentas de um agente é incomumente longo.

### Veja também

- [Escolhendo um Modelo](#choosing-a-model) - a maior alavanca sobre o custo.
- [Opções de Contexto](#context-options) - de onde vem o custo adicional.
- [Visão Geral de Orçamentos](#budgets-overview) - limites rígidos que evitam custos descontrolados.