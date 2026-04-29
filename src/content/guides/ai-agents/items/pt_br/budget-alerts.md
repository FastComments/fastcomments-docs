E-mails de alerta de orçamento são enviados quando os gastos de um agente ultrapassam uma porcentagem configurável do seu cap. Eles são enviados para as pessoas que são responsáveis pela fatura.

### Como os alertas funcionam

Each agent has an **Alert thresholds** field on the edit form. By default it is `80%` and `100%`. Você pode marcar ou desmarcar limiares individuais, e pode adicionar outras porcentagens.

Quando os gastos do agente em um determinado escopo (diário ou mensal) cruzam um limiar pela primeira vez naquele período, a plataforma envia um e-mail por destinatário. Cruzar o limiar novamente mais tarde no mesmo período (ex.: os gastos caíram abaixo de 80% e voltaram a ultrapassar) **não** reenvia.

Isto é por período: um novo reinício diário reinicia a lógica de cruzamento de limiares para esse dia.

### Tenant-scope alerts

O tenant (conta) tem seus próprios limites diários e mensais. Tenant-scope alerts disparam em limiares fixos (`80%` e `100%`). Estes não são configuráveis por agente porque se aplicam ao tenant inteiro.

### Recipients

Os alertas de orçamento são enviados para:

- Todo usuário marcado **Super admin** no tenant.
- Todo usuário marcado **Billing Admin** no tenant.

Isso inclui a união das duas funções - um usuário com ambas recebe um único e-mail.

### Por que ambos os papéis

Super admins tipicamente são os operadores que precisam saber quando um agente está atingindo seu cap. Billing admins são os responsáveis pela fatura e precisam saber sobre picos de custo independentemente de gerenciarem agentes no dia a dia. Para realmente editar o agente (aumentar o cap, pausar), o destinatário também precisa do papel **Customization Admin** - que controla o acesso à agent edit page.

### Desativação por usuário

Destinatários que optaram por não receber notificações administrativas em seu perfil são ignorados. Este é o mesmo interruptor de opt-out que controla outras notificações de admin.

Se **todos** os destinatários tiverem optado por não receber, o alerta é registrado (nível de aviso) e nenhum e-mail é enviado.

### Conteúdo do e-mail

O e-mail contém:

- The **agent display name** e nome interno.
- O **scope** que foi ultrapassado (ex.: "agent daily budget", "agent monthly budget", "account daily budget", "account monthly budget").
- A **threshold percentage** ultrapassada.
- **Usage** na moeda do tenant.
- **Cap** na moeda do tenant.
- Um **one-click signed login link** que leva o destinatário diretamente para:
  - A agent edit page, para alertas no escopo do agente.
  - A AI Agents list page, para alertas no escopo do tenant.

O link é pré-autenticado, então o destinatário está a um clique de aumentar o cap ou desativar o agente.

### Como os limiares disparam

A plataforma registra quais limiares já dispararam neste período, separadamente para o agente e para o tenant. Então:

- Cruzar 80% e depois 100% no mesmo período dispara ambos, em ordem.
- Ir direto de 0% para 100% em um grande salto dispara o limiar **mais alto** cruzado (100%), não 80%, então o alerta mais grave é o entregue.

### Quando você para de receber alertas

Se os gastos do agente nunca atingirem o próximo limiar neste período, você não recebe mais e-mails neste período. O próximo reinício diário (ou reinício mensal) limpa o rastreamento.

### Desativando alertas

Desmarque o limiar que você não deseja. Se você não quiser nenhum alerta em um agente específico, desmarque todas as porcentagens. Tenant-scope alerts não podem ser desativados por agente (são em nível de tenant).

### Veja também

- [Budgets Overview](#budgets-overview).
- [Drop Reasons](#drop-reasons) - o que acontece quando o cap é totalmente atingido.
- [Cost Model](#cost-model) - o que está sendo medido.