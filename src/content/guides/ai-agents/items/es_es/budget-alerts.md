Los correos de alertas de presupuesto se envían cuando el gasto de un agente cruza un porcentaje configurable de su límite. Van dirigidos a las personas responsables de la factura.

### Cómo funcionan las alertas

Cada agente tiene un campo **Umbrales de alerta** en el formulario de edición. Por defecto es `80%` y `100%`. Puedes marcar o desmarcar umbrales individuales, y puedes añadir otros porcentajes.

Cuando el gasto del agente en un ámbito determinado (diario o mensual) cruza un umbral por primera vez en ese periodo, la plataforma envía un correo por destinatario. Volver a cruzar el umbral más adelante en el mismo periodo (por ejemplo, si el gasto bajó del 80% y volvió a superarlo) **no** reenvía el correo.

Esto es por periodo: un nuevo reinicio diario reinicia la lógica de cruce de umbrales para ese día.

### Alertas de ámbito de tenant

El tenant (cuenta) tiene sus propios límites diarios y mensuales. Las alertas de ámbito de tenant se activan en umbrales fijos (`80%` y `100%`). Estos no son configurables por agente porque se aplican a todo el tenant.

### Destinatarios

Las alertas de presupuesto se envían a:

- Cada usuario marcado como **Super admin** en el tenant.
- Cada usuario marcado como **Billing Admin** en el tenant.

Esto incluye la unión de ambos roles: un usuario con ambos roles recibe un solo correo.

### Por qué ambos roles

Los super admins suelen ser los operadores que necesitan saber que un agente está alcanzando su límite. Los administradores de facturación son responsables de la factura y necesitan conocer los picos de coste independientemente de si gestionan agentes a diario. Para editar realmente el agente (aumentar el límite, pausarlo), el destinatario también necesita el rol **Customization Admin**, que controla el acceso a la página de edición del agente.

### Exclusión por usuario

Los destinatarios que hayan desactivado las notificaciones de administrador en su perfil son omitidos. Este es el mismo interruptor de exclusión que controla otras notificaciones de administrador.

Si **todos** los destinatarios están excluidos, la alerta se registra (nivel de advertencia) y no se envía ningún correo.

### Contenido del correo

El correo contiene:

- El **nombre visible del agente** y su nombre interno.
- El **ámbito** que cruzó el umbral (por ejemplo, "presupuesto diario del agente", "presupuesto mensual del agente", "presupuesto diario de la cuenta", "presupuesto mensual de la cuenta").
- El **porcentaje de umbral** cruzado.
- El **uso** en la moneda del tenant.
- El **límite** en la moneda del tenant.
- Un **enlace de inicio de sesión firmado de un clic** que lleva al destinatario directamente a:
  - La página de edición del agente, para alertas de ámbito de agente.
  - La página de lista de Agentes de IA, para alertas de ámbito de tenant.

El enlace viene preautenticado, por lo que el destinatario está a un clic de aumentar el límite o deshabilitar el agente.

### Cómo se activan los umbrales

La plataforma registra qué umbrales ya se han activado en este periodo, por separado para el agente y el tenant. Así:

- Cruzar el 80% y luego el 100% en el mismo periodo activa ambos, en orden.
- Pasar directamente del 0% al 100% en un solo salto activa el umbral cruzado **más alto** (100%), no el 80%, de modo que la alerta más severa es la que se entrega.

### Cuándo dejas de recibir alertas

Si el gasto del agente nunca alcanza el siguiente umbral en este periodo, no recibirás más correos en este periodo. El siguiente reinicio diario (o mensual) borra el registro.

### Deshabilitar alertas

Desmarca el umbral que no quieras. Si no quieres ninguna alerta en un agente específico, desmarca todos los porcentajes. Las alertas de ámbito de tenant no se pueden deshabilitar por agente (son para todo el tenant).

### Ver también

- [Visión general de presupuestos](#budgets-overview).
- [Motivos de descarte](#drop-reasons) - qué sucede cuando el límite se alcanza por completo.
- [Modelo de costes](#cost-model) - qué se está midiendo.