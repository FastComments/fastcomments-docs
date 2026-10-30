AI Agents are available on the **Flex** and **Pro** plans. The Creator plan does not include them.

### Límites a nivel de plan

Each plan tier sets:

- **Límites de presupuesto diarios y mensuales predeterminados.** Puedes reducir estos por agente; aumentar el límite por cuenta requiere un plan con un techo más alto. Ver [Resumen de presupuestos](#budgets-overview).

The exact numbers are shown on the [página de precios](https://fastcomments.com/traffic-pricing) and on your account's billing page. They are also shown inline on the agent edit form so you never have to leave the form to find your cap.

FastComments Pro includes $200/mo worth of AI usage. Flex is billed at the rate of $20 per million tokens for all models (currently either GLM 5.2 or gpt-oss-120B).

### La facturación debe ser válida

AI Agents only run when the tenant has **facturación válida archivada**. If the payment method becomes invalid, all agents are paused and the AI Agents page surfaces a banner directing whoever has the **Administrador de Facturación** role to update billing. Agents resume on their own once billing is restored - no replay or backfill of triggers that fired during the outage.

This is a hard prerequisite: token spend bills against your account, so the platform will not dispatch any LLM call without a working payment method.

### Quién puede gestionar los agentes

The agent admin pages are gated behind the **Customization Admin** dashboard role. **Administradores de Moderación de Comentarios** can review and decide approvals (see [Flujo de aprobación](#approval-workflow)) but cannot create or edit agents. **Administradores de Facturación** receive [correos electrónicos de alerta de presupuesto](#budget-alerts) regardless of whether they have agent access.