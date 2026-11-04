Desde la [página de Agentes de IA](https://fastcomments.com/auth/my-account/ai-agents) puedes crear un agente de dos maneras:

- **Desde una plantilla.** Haz clic en **Browse templates** y elige uno de los cuatro agentes iniciales integrados. El formulario aparece prellenado y el estado del agente es **Dry Run**. Consulta [Plantillas iniciales](#starter-templates).
- **Desde cero.** Haz clic en **Create new agent**. El formulario aparece en blanco.

En cualquier caso, el mismo formulario de edición es el que guardarás y editarás después. Esta página recorre el formulario de arriba a abajo.

### Básicos

- **Nombre interno.** Un identificador corto usado solo en los paneles de administración (historial de ejecuciones, analíticas, registros de auditoría). Funciona bien en minúsculas con guiones bajos: `moderator`, `welcome_greeter`. Si el nombre interno de una plantilla ya está en uso, el formulario añade automáticamente un sufijo (`tos_enforcer_2`, etc.).
- **Nombre para mostrar.** Se muestra públicamente cada vez que el agente publica un comentario. Esto es lo que ven tus lectores.
- **Estado.** Disabled, Dry Run o Enabled. Los agentes nuevos siempre tienen por defecto Dry Run. Consulta [Estados](#status-states).

### Modelo

Elige el LLM. Consulta [Elegir un modelo](#choosing-a-model).

### Presupuesto

Límites diarios y mensuales opcionales en la moneda de tu cuenta, además de una lista de verificación de **Alert thresholds** (80% y 100% por defecto). Consulta [Descripción general de presupuestos](#budgets-overview) y [Alertas de presupuesto](#budget-alerts).

### Personalidad

El **Initial prompt** es el prompt del sistema que define el tono, el rol y las reglas de decisión. Texto plano, sin sintaxis de plantillas. Consulta [Personalidad y el prompt inicial](#personality-prompt).

### Contexto

El conjunto de campos de Contexto tiene tres casillas de verificación, un área de texto de directrices y las entradas de alcance:

- Incluir el comentario padre y las respuestas previas en el mismo hilo.
- Incluir el factor de confianza del comentarista, la antigüedad de la cuenta, el historial de baneos y los comentarios recientes.
- Incluir el título de la página, el subtítulo, la descripción y las metaetiquetas.
- Un bloque de texto opcional de **Community guidelines** que se antepone a cada prompt.
- **Restrict to specific pages** - lista de permitidos de patrones de URL (uno por línea). Vacío significa para todo el tenant.
- **Restrict to specific locales** - lista de permitidos de configuraciones regionales mediante un selector de doble lista. Vacío significa todas las configuraciones regionales.

Más contexto produce mejores decisiones pero aumenta el costo de tokens por ejecución. Consulta [Opciones de contexto](#context-options), [Directrices de la comunidad](#community-guidelines) y [Alcance: filtros de URL y configuración regional](#scope-url-locale).

### Disparadores

Elige al menos un evento de la lista. Para los disparadores de umbral de votos y umbral de reportes también debes establecer el umbral. El campo opcional **Delay before running** pospone la ejecución después de que se dispare un disparador (útil para umbrales de reportes donde los votos aún se están estabilizando). Consulta [Descripción general de eventos de disparo](#triggers-overview) y [Disparadores diferidos](#trigger-deferred-delay).

### Llamadas a herramientas permitidas

Marca **Allow any tool calls** para exponer la paleta completa de herramientas. De lo contrario, marca las herramientas específicas que el agente puede usar - las herramientas no permitidas se eliminan de la paleta del modelo y se rechazan en el momento del despacho. La subsección **Ban options** condiciona las variantes destructivas de baneo (delete-all-comments, ban-by-IP) a una aceptación explícita. Consulta [Descripción general de llamadas a herramientas permitidas](#tools-overview) y [Herramienta: ban_user](#tool-ban-user).

### Aprobaciones

Marca las acciones que deben ser aprobadas por un humano antes de que el agente las ejecute. Las aprobaciones solo se aplican a las herramientas que el agente puede invocar; las herramientas no permitidas se rechazan directamente. En la región de la UE, **ban_user** está bloqueado en activado por el Artículo 17 de la Ley de Servicios Digitales. Consulta [Flujo de trabajo de aprobación](#approval-workflow) y [Cumplimiento del Artículo 17 del DSA de la UE](#eu-dsa-compliance).

### Notificaciones de aprobación

Si las aprobaciones están habilitadas, elige a quién se enviará un correo electrónico:

- **Todos los administradores y moderadores** - propietarios de cuentas, superadministradores y administradores moderadores de comentarios.
- **Usuarios específicos** - seleccionados manualmente desde un selector de doble lista.

La frecuencia de entrega individual de cada revisor (inmediata, resumen por hora, resumen diario) se establece en su propio perfil. Consulta [Notificaciones de aprobación](#approval-notifications).

### Estadísticas

Solo lectura. Total de ejecuciones, marca de tiempo de la última ejecución y el ID del comentario más reciente que escribió el agente (si lo hay).

### Guardar

Haz clic en **Save agent**. La página redirige de vuelta a la lista de agentes. Los agentes nuevos son inmediatamente elegibles para recibir disparadores en modo dry-run.

### Edición posterior

Cada fila de la página de lista de agentes expone acciones por agente: **Edit**, **Clone**, **Runs**, **Replays**, **Test run**, **Analytics**, **Approvals** y **Delete**. Editar un agente no afecta retroactivamente las ejecuciones ya registradas - el historial se conserva. Las instantáneas de repetición también congelan la configuración del agente en el momento en que se inició la repetición, por lo que los resultados de una repetición guardada permanecen reproducibles incluso después de editar el prompt.