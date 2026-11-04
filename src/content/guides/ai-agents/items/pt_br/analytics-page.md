Analytics é o painel entre agentes. Acessível pela página de AI Agents através da aba **Analytics** (em todo o tenant) ou por agente através do botão **Analytics** na linha de cada agente.

### Filtro

Um dropdown no topo - **All agents** ou um agente específico. O restante da página é reescopado de acordo.

### Uso de orçamento

Quatro barras de progresso mostrando o gasto do período atual em relação ao limite:

- **Agent today** (quando filtrado para um agente específico) - limite diário do agente.
- **Agent this month** - limite mensal do agente.
- **Account today** - limite diário do tenant.
- **Account this month** - limite mensal do tenant.

Quando um limite não está definido, a barra exibe "(no cap set)" e mostra o gasto bruto.

### Custo diário (últimos 30 dias)

Uma tabela de custo por dia na moeda do seu tenant para o escopo selecionado. Útil para identificar:

- **Picos repentinos de custo** - geralmente de um loop descontrolado ou um comentário viral disparando gatilhos em cascata.
- **Deriva de custo** - custo diário aumentando gradualmente conforme sua comunidade cresce.

### Ações tomadas

Um detalhamento dos tipos de ação no mês atual - "Wrote a comment: 47", "Marked a comment as spam: 12", e assim por diante. Útil para verificar se o agente está fazendo o que você esperava.

### Gatilhos ignorados (este mês)

Contagens agrupadas por [motivo de descarte](#drop-reasons):

- Acima do limite diário do agente / mensal do agente / diário da conta / mensal da conta.
- Limitado por taxa (rate-limited).
- Concorrência saturada.

Se você vê descartes aqui, seu agente está atingindo um limite de orçamento ou de taxa e está perdendo gatilhos nos quais ele teria agido. Veja [Motivos de Descarte](#drop-reasons).

### Dry-run vs live (este mês)

- **Enabled runs** - contagem de execuções que tomaram ações reais neste mês.
- **Dry runs** - contagem de execuções em modo dry-run neste mês.

Um sinal útil de ajuste: um agente novo que ainda não foi promovido para Enabled mostrará apenas dry runs. Um agente em Enabled com contagens zeradas nesta seção está ocioso - ou não está sendo acionado, ou está sendo filtrado pelo escopo, ou seus gatilhos não estão configurados corretamente.

### Principais agentes por custo mensal

Quando o filtro é **All agents**, a página lista os agentes classificados pelo custo acumulado do mês. Identificar seu agente mais caro é o primeiro passo para otimizar custos - geralmente a resposta é "restringir suas [opções de contexto](#context-options)" ou "diminuir seu [limite de orçamento](#budgets-overview)".

### Agentes no limite ou próximos dele

Detalhamento por agente daqueles cujo gasto está no limite ou próximo dos limites por agente no período atual:

- **near cap** - acima de uma porcentagem configurável do limite.
- **over cap** - efetivamente limitado, com `{count} dropped` gatilhos nesse período.

Clique no agente nesta tabela para aumentar o limite, restringir o escopo ou pausá-lo.

### Resumo da conta

Quando o filtro é **All agents**:

- **Triggers today** - contagem.
- **Triggers this month** - contagem.
- Para cada um: um sufixo `dropped` mostrando quantos foram ignorados.

### Moeda

Todos os valores monetários são exibidos na moeda do seu tenant.

### O que esta página não faz

- Não mostra **detalhamentos de custo por ação** - eles estão em [Run Detail View](#run-detail-view).
- Não mostra **transcrições** ou **respostas de LLM**.
- Não permite tomar ações nos agentes - edição, pausa e exclusão são feitas na lista de agentes / página de edição.