AI Agents are available on the **Flex** and **Pro** plans. The Creator plan does not include them.

### Limites por plano

Cada nível de plano define:

- **Limites padrão de orçamento diário e mensal.** Você pode reduzir esses limites por agente; aumentar o limite por conta requer um plano com um teto mais alto. Veja [Visão geral de orçamentos](#budgets-overview).

Os números exatos são mostrados na [página de preços](https://fastcomments.com/traffic-pricing) e na página de faturamento da sua conta. Eles também são exibidos inline no formulário de edição do agente para que você nunca precise sair do formulário para encontrar seu limite.

FastComments Pro inclui $200/mês de uso de IA. Flex é faturado à taxa de $20 por milhão de tokens para todos os modelos (atualmente GLM 5.1 ou gpt-oss-120B-turbo).

### Faturamento deve ser válido

AI Agents só são executados quando o locatário possui **dados de faturamento válidos**. Se o método de pagamento se tornar inválido, todos os agentes são pausados e a página AI Agents exibe um banner direcionando quem tiver a função **Billing Admin** a atualizar o faturamento. Os agentes retomam automaticamente quando o faturamento é restaurado — não há replay ou backfill de triggers que foram acionados durante a interrupção.

Isto é um pré-requisito rígido: o gasto com tokens é faturado contra sua conta, portanto a plataforma não disparará qualquer chamada a LLM sem um método de pagamento funcionando.

### Quem pode gerenciar agentes

As páginas de administração de agentes são restritas pela função de painel **Customization Admin**. **Comment Moderator Admins** podem revisar e decidir aprovações (veja [Fluxo de Aprovação](#approval-workflow)) mas não podem criar ou editar agentes. **Billing Admins** recebem [emails de alerta de orçamento](#budget-alerts) independentemente de terem acesso aos agentes.