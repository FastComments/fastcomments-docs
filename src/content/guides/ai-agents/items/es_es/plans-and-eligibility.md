Los Agentes de IA están disponibles en los planes **Flex** y **Pro**. El plan Creator no los incluye.

### Límites por plan

Cada nivel de plan establece:

- **Límites presupuestarios diarios y mensuales predeterminados.** Puedes reducirlos por agente; aumentar el límite por cuenta requiere un plan con un techo más alto. Consulta [Descripción general de presupuestos](#budgets-overview).

Las cifras exactas se muestran en la [página de precios](https://fastcomments.com/traffic-pricing) y en la página de facturación de tu cuenta. También se muestran en línea en el formulario de edición del agente, para que nunca tengas que salir del formulario para encontrar tu límite.

FastComments Pro incluye $200/mes de uso de IA. Flex se factura a una tarifa de $20 por millón de tokens para todos los modelos (actualmente GLM 5.2 o gpt-oss-120B).

### La facturación debe ser válida

Los Agentes de IA solo se ejecutan cuando el tenant tiene **facturación válida registrada**. Si el método de pago deja de ser válido, todos los agentes se pausan y la página de Agentes de IA muestra un banner que dirige a quien tenga el rol de **Administrador de Facturación** a actualizar la facturación. Los agentes se reanudan por sí solos una vez restaurada la facturación - sin repetición ni relleno de los disparadores que se activaron durante la interrupción.

Este es un requisito previo estricto: el gasto de tokens se factura a tu cuenta, por lo que la plataforma no despachará ninguna llamada LLM sin un método de pago funcional.

### Quién puede gestionar los agentes

Las páginas de administración de agentes están restringidas al rol de panel **Administrador de Personalización**. Los **Administradores Moderadores de Comentarios** pueden revisar y decidir aprobaciones (ver [Flujo de Aprobación](#approval-workflow)) pero no pueden crear ni editar agentes. Los **Administradores de Facturación** reciben [correos de alerta de presupuesto](#budget-alerts) independientemente de si tienen acceso a los agentes.