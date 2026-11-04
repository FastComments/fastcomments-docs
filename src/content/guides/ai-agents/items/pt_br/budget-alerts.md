Os e-mails de alerta de orçamento são disparados quando o gasto de um agente cruza uma porcentagem configurável do seu limite. Eles vão para as pessoas que respondem pela conta.

### Como os alertas funcionam

Cada agente tem um campo **Alert thresholds** no formulário de edição. Por padrão, ele contém `80%` e `100%`. Você pode marcar ou desmarcar limites individuais e pode adicionar outras porcentagens.

Quando o gasto do agente em um determinado escopo (diário ou mensal) cruza um limite pela primeira vez naquele período, a plataforma envia um e-mail por destinatário. Cruzar o limite novamente mais tarde no mesmo período (por exemplo, o gasto caiu abaixo de 80% e voltou a ultrapassá-lo) **não** reenvia o e-mail.

Isso é por período: um novo reset diário reinicia a lógica de cruzamento de limites para aquele dia.

### Alertas de escopo do tenant

O tenant (conta) tem seus próprios limites diários e mensais. Os alertas de escopo do tenant são disparados em limites fixos (`80%` e `100%`). Eles não são configuráveis por agente porque se aplicam ao tenant inteiro.

### Destinatários

Os alertas de orçamento são enviados para:

- Todo usuário marcado como **Super admin** no tenant.
- Todo usuário marcado como **Billing Admin** no tenant.

Isso inclui a união dos dois papéis - um usuário com ambos os papéis recebe um único e-mail.

### Por que ambos os papéis

Super admins são tipicamente os operadores que precisam saber que um agente está atingindo seu limite. Os administradores de faturamento respondem pela fatura e precisam saber sobre picos de custo, independentemente de gerenciarem agentes no dia a dia. Para realmente editar o agente (aumentar o limite, pausá-lo), o destinatário também precisa do papel **Customization Admin** - que controla o acesso à página de edição do agente.

### Opt-out por usuário

Destinatários que desativaram as notificações de administrador em seu perfil são ignorados. Este é o mesmo interruptor de opt-out que controla outras notificações de administrador.

Se **todos** os destinatários estiverem com opt-out ativado, o alerta é registrado (nível de aviso) e nenhum e-mail é enviado.

### Conteúdo do e-mail

O e-mail contém:

- O **nome de exibição do agente** e o nome interno.
- O **escopo** que cruzou o limite (por exemplo, "agent daily budget", "agent monthly budget", "account daily budget", "account monthly budget").
- A **porcentagem do limite** cruzada.
- **Uso** na moeda do tenant.
- **Limite** na moeda do tenant.
- Um **link de login assinado de um clique** que leva o destinatário diretamente para:
  - A página de edição do agente, para alertas de escopo de agente.
  - A página de lista de AI Agents, para alertas de escopo de tenant.

O link é pré-autenticado, então o destinatário está a um clique de aumentar o limite ou desabilitar o agente.

### Como os limites são disparados

A plataforma rastreia quais limites já foram disparados neste período, separadamente para o agente e para o tenant. Então:

- Cruzar 80% e depois 100% no mesmo período dispara ambos, em ordem.
- Ir diretamente de 0% para 100% em um grande salto dispara o limite cruzado **mais alto** (100%), não 80%, de modo que o alerta mais severo é o entregue.

### Quando você para de receber alertas

Se o gasto do agente nunca atingir o próximo limite neste período, você não receberá mais e-mails neste período. O próximo reset diário (ou mensal) limpa o rastreamento.

### Desabilitando alertas

Desmarque o limite que você não deseja. Se você não quiser nenhum alerta em um agente específico, desmarque todas as porcentagens. Os alertas de escopo do tenant não podem ser desabilitados por agente (são para todo o tenant).

### Veja também

- [Budgets Overview](#budgets-overview).
- [Drop Reasons](#drop-reasons) - o que acontece quando o limite é totalmente atingido.
- [Cost Model](#cost-model) - o que está sendo medido.