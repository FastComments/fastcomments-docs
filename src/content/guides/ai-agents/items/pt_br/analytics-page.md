Analytics é o painel entre agentes. Acessível a partir da página Agentes de IA via a aba **Analytics** (no nível do tenant) ou por agente via o botão **Analytics** na linha de cada agente.

### Filter

Um dropdown no topo - **Todos os agentes** ou um agente específico. O restante da página ajusta o escopo de acordo.

### Budget usage

Quatro barras de progresso mostrando o gasto do período atual em relação ao limite:

- **Agent today** (quando filtrado para um agente específico) - limite diário do agente.
- **Agent this month** - limite mensal do agente.
- **Account today** - limite diário do tenant.
- **Account this month** - limite mensal do tenant.

Quando um limite não está definido, a barra mostra "(no cap set)" e exibe o gasto bruto.

### Daily cost (last 30 days)

Uma tabela do custo por dia na moeda do seu tenant para o escopo selecionado. Útil para identificar:

- **Picos súbitos de custo** - geralmente causados por um loop descontrolado ou um comentário viral que dispara muitos gatilhos.
- **Deriva de custo** - aumento gradual do custo diário conforme sua comunidade cresce.

### Actions taken

Um detalhamento dos tipos de ação ao longo do mês atual - "Escreveu um comentário: 47", "Marcou um comentário como spam: 12" e assim por diante. Útil para checar se o agente está fazendo o que você esperava.

### Triggers skipped (this month)

Contagens agrupadas por [motivo de descarte](#drop-reasons):

- Acima do limite diário do agente / mensal do agente / diário da conta / mensal da conta.
- Limitado por taxa.
- Concorrência saturada.

Se você vir descarte aqui, seu agente está atingindo um limite de orçamento ou de taxa e está perdendo gatilhos que normalmente executaria. Veja [Motivos de Descarte](#drop-reasons).

### Dry-run vs live (this month)

- **Execuções habilitadas** - contagem de execuções que realizaram ações reais este mês.
- **Execuções em modo simulação** - contagem de execuções em modo simulação este mês.

Um sinal útil para ajuste: um agente recém-criado que ainda não foi promovido a Habilitado exibirá apenas execuções em modo simulação. Um agente Habilitado com todas as contagens zeradas nesta seção está inativo — ou não está sendo acionado, ou está sendo excluído do escopo, ou seus gatilhos não estão configurados corretamente.

### Top agents by monthly cost

Quando o filtro está em **Todos os agentes**, a página lista agentes classificados pelo custo acumulado no mês até a data. Identificar seu agente mais caro é o primeiro passo na otimização de custos - geralmente a resposta é "apertar suas [opções de contexto](#context-options)" ou "reduzir seu [limite de orçamento](#budgets-overview)".

### Agents at or near their cap

Detalhamento por agente dos que tiveram gasto no limite ou próximos dos limites por agente no período atual:

- **near cap** - acima de uma porcentagem configurável do limite.
- **over cap** - efetivamente no limite, com `{count} dropped` gatilhos nesse período.

Clique no agente nesta tabela para aumentar o limite, reduzir o escopo ou pausá-lo.

### Account summary

Quando o filtro está em **Todos os agentes**:

- **Triggers today** - contagem.
- **Triggers this month** - contagem.
- Para cada um: um sufixo `dropped` mostrando quantos foram ignorados.

### Currency

Todos os valores monetários são exibidos na moeda do seu tenant.

### What this page does not do

- Não mostra **detalhamento de custo por ação** - esses estão em [Visualização de Detalhe da Execução](#run-detail-view).
- Não mostra **transcrições** ou **respostas do LLM**.
- Não permite que você tome ações nos agentes - editar, pausar, excluir são feitos na lista de agentes / página de edição.