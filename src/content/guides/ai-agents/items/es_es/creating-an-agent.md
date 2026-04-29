Desde la [página de Agentes de IA](https://fastcomments.com/auth/my-account/ai-agents) puedes crear un agente de dos maneras:

- **Desde una plantilla.** Haz clic en **Browse templates** y elige uno de los cuatro agentes iniciales incorporados. El formulario aparece con datos precargados y el estado del agente es **Dry Run**. Vea [Starter Templates](#starter-templates).
- **Desde cero.** Haz clic en **Create new agent**. El formulario aparece en blanco.

De cualquier modo, el mismo formulario de edición es lo que guardarás y editarás después. Esta página recorre el formulario de arriba a abajo.

### Basics

- **Internal name.** Un identificador corto usado solo en los paneles de administración (historial de ejecuciones, analítica, registros de auditoría). Funciona bien en minúsculas con guiones bajos: `moderator`, `welcome_greeter`. Si el nombre interno de una plantilla ya está tomado, el formulario le añade un sufijo automáticamente (`tos_enforcer_2`, etc.).
- **Display name.** Se muestra públicamente cada vez que el agente publica un comentario. Esto es lo que ven tus lectores.
- **Status.** Disabled, Dry Run, or Enabled. Los agentes nuevos siempre por defecto están en Dry Run. Vea [Status States](#status-states).

### Model

Elige el LLM. Vea [Choosing a Model](#choosing-a-model).

### Budget

Topes diarios y mensuales opcionales en la moneda de tu cuenta, además de una lista de verificación de **Alert thresholds** (por defecto 80% y 100%). Vea [Budgets Overview](#budgets-overview) y [Budget Alerts](#budget-alerts).

### Personality

El **Initial prompt** es el prompt de sistema que define el tono, el rol y las reglas de decisión. Texto plano, sin sintaxis de plantillas. Vea [Personality and the Initial Prompt](#personality-prompt).

### Context

El conjunto de campos Context tiene tres casillas, un área de texto para directrices y los campos de alcance:

- Incluir el comentario padre y las respuestas previas en el mismo hilo.
- Incluir el factor de confianza del comentarista, la antigüedad de la cuenta, historial de prohibiciones y comentarios recientes.
- Incluir el título de la página, subtítulo, descripción y meta tags.
- Un bloque de texto opcional de **Community guidelines** que se antepone a cada prompt.
- **Restrict to specific pages** - lista blanca de patrones de URL (uno por línea). Vacío significa a nivel de tenant.
- **Restrict to specific locales** - lista blanca de locales mediante un selector de doble lista. Vacío significa todos los locales.

Más contexto produce mejores decisiones pero aumenta el coste en tokens por ejecución. Vea [Context Options](#context-options), [Community Guidelines](#community-guidelines) y [Scope: URL and Locale Filters](#scope-url-locale).

### Triggers

Elige al menos un evento de la lista. Para los triggers de vote-threshold y flag-threshold también debes establecer el umbral. El campo opcional **Delay before running** difiere la ejecución después de que un trigger se dispare (útil para umbrales de flags donde los votos aún se están estabilizando). Vea [Trigger Events Overview](#triggers-overview) y [Deferred Triggers](#trigger-deferred-delay).

### Allowed tool calls

Marca **Allow any tool calls** para exponer la paleta completa de herramientas. De lo contrario, marca las herramientas específicas que el agente puede usar; las herramientas no permitidas se eliminan de la paleta del modelo y son rechazadas en tiempo de despacho. La subsección **Ban options** restringe las variantes destructivas de baneo (delete-all-comments, ban-by-IP) detrás de opt-ins explícitos. Vea [Allowed Tool Calls Overview](#tools-overview) y [Herramienta: ban_user](#tool-ban-user).

### Approvals

Marca las acciones que deben ser aprobadas por un humano antes de que el agente las ejecute. Las aprobaciones solo aplican a las herramientas que el agente tiene permitido invocar; las herramientas no permitidas son rechazadas de plano. En la región de la UE, **ban_user** está bloqueada por el Artículo 17 del Digital Services Act. Vea [Approval Workflow](#approval-workflow) y [Cumplimiento del Artículo 17 de la DSA de la UE](#eu-dsa-compliance).

### Approval notifications

Si las aprobaciones están habilitadas, elige a quién se le envía un correo:

- **All admins and moderators** - propietarios de la cuenta, superadministradores y administradores moderadores de comentarios.
- **Specific users** - seleccionados manualmente desde un selector de doble lista.

La frecuencia de entrega individual de cada revisor (inmediata, resumen por hora, resumen diario) se configura en su propio perfil. Vea [Approval Notifications](#approval-notifications).

### Stats

Solo lectura. Total de ejecuciones, marca temporal de la última ejecución y el ID del comentario más reciente que escribió el agente (si existe).

### Save

Haz clic en **Save agent**. La página redirige de vuelta a la lista de agentes. Los agentes nuevos son inmediatamente elegibles para recibir triggers en dry-run.

### Editing later

Cada fila en la página de lista de agentes expone acciones por agente: **Edit**, **Clone**, **Runs**, **Replays**, **Test run**, **Analytics**, **Approvals**, y **Delete**. Editar un agente no afecta retroactivamente a ejecuciones ya registradas: el historial se preserva. Las instantáneas de replay también congelan la configuración del agente en el momento en que se inició el replay, por lo que los resultados de un replay guardado siguen siendo reproducibles incluso después de editar el prompt.