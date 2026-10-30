AI Agents estão disponíveis nos planos **Flex** e **Pro**. O plano Creator não os inclui.

### Limites de nível de plano

- **Limites padrão diários e mensais de orçamento.** Você pode reduzir esses limites por agente; aumentar o limite por conta requer um plano com um teto mais alto. Veja [Visão geral dos orçamentos](#budgets-overview).

Os números exatos são exibidos na [página de preços](https://fastcomments.com/traffic-pricing) e na página de faturamento da sua conta. Eles também são mostrados inline no formulário de edição do agente, para que você nunca precise sair do formulário para encontrar seu limite.

O FastComments Pro inclui US$200/mês de uso de IA. O Flex é cobrado à taxa de US$20 por milhão de tokens para todos os modelos (atualmente GLM 5.2 ou gpt-oss-120B).

### O faturamento deve ser válido

Os Agentes de IA só funcionam quando o locatário tem **faturamento válido registrado**. Se o método de pagamento se tornar inválido, todos os agentes são pausados e a página de Agentes de IA exibe um banner direcionando quem tem a função **Administrador de Faturamento** a atualizar o faturamento. Os agentes retomam automaticamente assim que o faturamento for restaurado – sem reprodução ou preenchimento retroativo dos gatilhos que foram disparados durante a interrupção.

Isso é um pré‑requisito rígido: o gasto de tokens é cobrado na sua conta, portanto a plataforma não enviará nenhuma chamada de LLM sem um método de pagamento funcional.

### Quem pode gerenciar agentes

As páginas de administração de agentes são restritas à função de painel **Administrador de Personalização**. **Administradores de Moderação de Comentários** podem revisar e decidir aprovações (veja [Fluxo de aprovação](#approval-workflow)) mas não podem criar ou editar agentes. **Administradores de Faturamento** recebem [e‑mails de alerta de orçamento](#budget-alerts) independentemente de terem acesso aos agentes.