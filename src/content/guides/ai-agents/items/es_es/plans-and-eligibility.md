AI Agents están disponibles en los planes **Flex** y **Pro**. El plan **Creator** no los incluye.

### Límites a nivel de plan

Cada nivel de plan establece:

- **Límites predeterminados de presupuesto diario y mensual.** Puedes reducirlos por agente; aumentar el límite por cuenta requiere un plan con un techo superior. Consulta [Resumen de presupuestos](#budgets-overview).

Los números exactos aparecen en la [página de precios](https://fastcomments.com/traffic-pricing) y en la página de facturación de tu cuenta. También se muestran en línea en el formulario de edición del agente para que nunca tengas que abandonar el formulario para encontrar tu límite.

FastComments Pro incluye $200/mes de uso de IA. Flex se factura a la tasa de $20 por millón de tokens para todos los modelos (actualmente GLM 5.1 o gpt-oss-120B-turbo).

### La facturación debe ser válida

AI Agents solo se ejecutan cuando el tenant tiene **facturación válida registrada**. Si el método de pago queda inválido, todos los agentes se pausan y la página de AI Agents muestra un banner que indica a quien tenga el rol de **Billing Admin** que actualice la facturación. Los agentes se reanudarán por sí solos una vez que se restaure la facturación; no hay replay ni backfill de los triggers que se activaron durante la interrupción.

Esto es un requisito estricto: el gasto de tokens se factura a su cuenta, por lo que la plataforma no realizará ninguna llamada a LLM sin un método de pago operativo.

### Quién puede gestionar agentes

Las páginas de administración de agentes están restringidas al rol de panel de control **Customization Admin**. **Comment Moderator Admins** pueden revisar y decidir aprobaciones (ver [Flujo de Aprobación](#approval-workflow)) pero no pueden crear ni editar agentes. **Billing Admins** reciben [correos electrónicos de alerta de presupuesto](#budget-alerts) independientemente de si tienen acceso a los agentes.