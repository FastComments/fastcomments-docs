Todo agente tem limites de gasto. A plataforma para de despachar o agente quando um limite é alcançado e retoma quando o período reinicia.

### Dois escopos, dois períodos

Existem quatro limites no total - dois escopos (por agente, por tenant) cruzados com dois períodos (diário, mensal).

| Scope | Period | Where you set it |
|---|---|---|
| Per-agent daily | UTC day | Agent edit form -> **Budget** -> **Daily budget** |
| Per-agent monthly | calendar month | Agent edit form -> **Budget** -> **Monthly budget** |
| Per-tenant daily | UTC day | Plan-derived (no separate user-facing input) |
| Per-tenant monthly | calendar month | Plan-derived (no separate user-facing input) |

Um trigger só é despachado se **os quatro limites** permitirem. O primeiro limite a ser esgotado é o que faz o trigger ser derrubado.

### Moeda

Os orçamentos por agente são inseridos na moeda da sua conta.

### O que acontece quando um limite é alcançado

- O trigger é registrado como **derrubado** com um [motivo de descarte](#drop-reasons) como `agentDaily` ou `tenantMonthly`.
- A contagem de derrubados aparece na [Página de Analytics](#analytics-page) em "Triggers ignorados (este mês)".
- Nenhuma chamada LLM é feita; nenhum token é gasto no próprio trigger derrubado.
- O status do agente não muda — ele apenas não pode despachar até que o período reinicie.

### Reinício do período

- Os limites **diários** são resetados à meia-noite UTC.
- Os limites **mensais** são resetados no início de cada mês do calendário, UTC.

Não há transferência do orçamento não utilizado para o próximo período.

### Limite rígido vs avisos suaves

Os limites são **rígidos**. Não existe modo de "ultrapassar em 10% com aviso". Quando o limite é alcançado, o despacho para.

A parte "suave" são os e-mails de [Alertas de Orçamento](#budget-alerts) — você recebe um e-mail em limiares configuráveis (padrão 80% e 100%) para que possa aumentar o limite antes que o tráfego comece a cair.

### Onde ver o uso atual

- [Página de Analytics](#analytics-page) — uso do orçamento por agente e por tenant com marcadores de limite.
- A seção **Stats** do formulário de edição do agente.
- A visualização em lista (contagem de aprovações pendentes e execuções recentes no cartão do agente).

### Como escolher um orçamento

Algumas regras práticas:

- **Um agente novo** — determine um orçamento. Observe o [Histórico de Execuções](#run-history) por uma semana. Ajuste com base no custo observado por execução × volume de gatilhos esperado.
- **Um agente de alto volume** (por exemplo, trigger de novo comentário em um site com muito tráfego) — o limite diário é o que pega um loop descontrolado. Escolha um limite diário que seja 2–3× seu gasto diário esperado para que um dia movimentado caiba confortavelmente dentro dele.
- **Um agente de sumarização ou que exige muito contexto** — o custo por execução é alto. Defina um limite diário mais restrito para evitar que um dia ruim consuma o mensal.

### Bypass de orçamento para replays

[Test runs / replays](#test-runs-replays) estão sujeitos ao seu **próprio** limite rígido (definido no formulário de replay, separado dos limites diário/mensal do agente), E aos limites do agente e do tenant. Qualquer um que for atingido primeiro interrompe o replay.

### Veja também

- [Alertas de Orçamento](#budget-alerts) para as notificações por e-mail.
- [Modelo de custo](#cost-model) para como a plataforma converte tokens em dólares.
- [Motivos de descarte](#drop-reasons) para a lista completa de razões pelas quais um trigger não é executado.