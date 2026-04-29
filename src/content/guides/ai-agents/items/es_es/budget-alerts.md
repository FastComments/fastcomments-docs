---
Los correos electrónicos de alerta presupuestaria se envían cuando el gasto de un agente supera un porcentaje configurable de su límite. Van a las personas que son responsables de la factura.

### Cómo funcionan las alertas

Cada agente tiene un campo **Alert thresholds** en el formulario de edición. Por defecto es `80%` y `100%`. Puedes marcar o desmarcar umbrales individuales, y puedes añadir otros porcentajes.

Cuando el gasto del agente en un ámbito determinado (diario o mensual) supera un umbral por primera vez en ese periodo, la plataforma envía un correo electrónico por destinatario. Superar el umbral de nuevo más tarde en el mismo periodo (p. ej., el gasto bajó por debajo del 80% y volvió a subir) **no** provoca un reenvío.

Esto es por periodo: un nuevo reinicio diario reinicia la lógica de cruce de umbrales para ese día.

### Tenant-scope alerts

El tenant (cuenta) tiene sus propios límites diarios y mensuales. Las alertas a nivel de tenant se disparan en umbrales fijos (`80%` y `100%`). Estos no son configurables por agente porque se aplican a todo el tenant.

### Destinatarios

- Cada usuario marcado **Super admin** en el tenant.
- Cada usuario marcado **Billing Admin** en el tenant.

Esto incluye la unión de los dos roles - un usuario con ambos roles recibe un solo correo electrónico.

### Por qué ambos roles

Los **Super admin** suelen ser los operadores que necesitan saber que un agente está alcanzando su límite. Los **Billing Admin** son los responsables de la factura y necesitan conocer picos de coste independientemente de si gestionan los agentes en el día a día. Para editar realmente el agente (aumentar el límite, pausarlo), el destinatario también necesita el rol **Customization Admin** - que restringe la página de edición del agente.

### Exclusión por usuario

Los destinatarios que hayan optado por no recibir notificaciones de administración en su perfil son omitidos. Es el mismo interruptor de exclusión que controla otras notificaciones de administración.

Si **todos** los destinatarios han optado por no recibirlas, la alerta se registra (nivel de advertencia) y no se envía ningún correo.

### Contenido del correo electrónico

El correo contiene:

- El **nombre para mostrar del agente** y el nombre interno.
- El **ámbito** que se cruzó (p. ej., "agent daily budget", "agent monthly budget", "account daily budget", "account monthly budget").
- El **porcentaje de umbral** que se cruzó.
- **Uso** en la moneda del tenant.
- **Límite** en la moneda del tenant.
- Un **enlace de inicio de sesión firmado de un clic** que lleva al destinatario directamente a:
  - La página de edición del agente, para alertas a nivel de agente.
  - La página de lista AI Agents, para alertas a nivel de tenant.

El enlace está preautenticado, por lo que el destinatario queda a un clic de aumentar el límite o desactivar el agente.

### Cómo se disparan los umbrales

La plataforma rastrea qué umbrales ya se han disparado en este periodo, por separado para el agente y el tenant. Por lo tanto:

- Cruzar 80% y luego 100% en el mismo periodo dispara ambos, en orden.
- Ir directamente de 0% a 100% en un salto grande dispara el umbral cruzado **más alto** (100%), no el 80%, por lo que se entrega la alerta más grave.

### Cuándo dejas de recibir alertas

Si el gasto del agente nunca alcanza el siguiente umbral en este periodo, no recibirás más correos durante este periodo. El siguiente reinicio diario (o reinicio mensual) borra el seguimiento.

### Desactivar alertas

Desmarca el umbral que no quieras. Si no deseas ninguna alerta en un agente específico, desmarca todos los porcentajes. Las alertas a nivel de tenant no pueden desactivarse por agente (son a nivel de tenant).

### Véase también

- [Budgets Overview](#budgets-overview).
- [Drop Reasons](#drop-reasons) - qué ocurre cuando se alcanza completamente el límite.
- [Cost Model](#cost-model) - lo que se está midiendo.

---