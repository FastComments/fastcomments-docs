Cuando el agente encola una aprobación, la plataforma notifica a los revisores por correo electrónico. Dos ajustes en el formulario de edición controlan esto: **quién** recibe la notificación y **con qué frecuencia**.

### Quién: modo de notificación

Dos modos:

- **Todos los administradores y moderadores** (predeterminado) - cada propietario de cuenta, superadministrador y administrador moderador de comentarios del tenant es un revisor candidato.
- **Usuarios específicos** - seleccione manualmente una lista mediante un selector de doble lista en el formulario de edición.

En cualquier caso, un revisor candidato debe tener una cuenta en el tenant y una dirección de correo electrónico válida para recibir notificaciones.

### Con qué frecuencia: frecuencia por usuario

El **propio perfil** de cada revisor candidato establece su frecuencia personal de notificaciones para aprobaciones del agente:

- **Inmediato** (predeterminado) - un correo por cada aprobación pendiente, enviado tan pronto como se cree la aprobación.
- **Cada hora** - un correo de resumen por hora que resume todas las aprobaciones encoladas durante esa hora.
- **Diario** - un correo de resumen cada 24 horas.
- **Desactivado** - ningún correo. El usuario aún puede revisar las aprobaciones a través de la interfaz de la bandeja de entrada; simplemente no se le notifica.

El usuario cambia este ajuste en su propio perfil, no en el formulario de edición del agente. Esto es intencional: un tenant podría tener diez agentes, y un moderador no debería tener que establecer su frecuencia preferida en cada agente de forma independiente.

### Tareas cron que generan los resúmenes

- **`hourly-agent-approval-digest`** - se ejecuta cada hora, agrupa las aprobaciones encoladas desde el último resumen de cada usuario y envía un correo por usuario.
- **`daily-agent-approval-digest`** - lo mismo, diariamente.
- **`agent-approval-reaper`** - elimina las aprobaciones que tengan más de 90 días, independientemente de su estado.

Las tareas cron de resumen por hora y por día están limitadas por destinatario: un usuario con frecuencia horaria es procesado por la tarea cron horaria y omitido por la diaria (y viceversa). Los usuarios con frecuencia Inmediato son notificados por la ruta de código approval-create, no por las tareas cron.

### Estado de deduplicación

La plataforma rastrea qué usuarios ya han recibido un correo sobre cada aprobación. Una vez que un usuario ha sido notificado (inmediatamente o en un resumen), no volverá a recibir un correo por la misma aprobación, incluso si cambia su frecuencia de Inmediato a Diario a mitad del ciclo.

### Aprobar desde el correo

Cada correo de notificación contiene un enlace de inicio de sesión firmado de un solo clic que lleva al revisor directamente a la página de detalles de la aprobación, ya autenticado. Desde allí pueden aprobar, rechazar o abrir el flujo de [Refinar prompts](#refining-prompts).

### Qué ocurre si no existen administradores

Si `notifyMode` es `All admins and moderators` pero el tenant no tiene super administradores, administradores moderadores de comentarios o propietarios de cuenta con correos electrónicos válidos, la plataforma registra una advertencia y la aprobación aún se encola: simplemente nadie recibe una notificación. Permanecerá en la bandeja de entrada hasta que alguien lo revise.

Si `notifyMode` es `Specific users` pero no ha seleccionado ningún usuario, el resultado es el mismo.

### Qué ocurre si las notificaciones de facturación están deshabilitadas

[Alertas de presupuesto](#budget-alerts) — los correos relacionados con el presupuesto — se envían a los administradores de facturación **independientemente de la preferencia de notificación por usuario**. Esto es intencional: los sobrecostes de presupuesto afectan al coste y el responsable de facturación necesita saberlo.

Las notificaciones de aprobación respetan únicamente la configuración de frecuencia per-usuario de agent-approval. No verifican el opt-out más amplio de admin-notifications: un usuario que haya optado por no recibir notificaciones de administrador seguirá recibiendo correos de aprobación si está en la lista de revisores, a menos que su frecuencia de agent-approval esté configurada como **Desactivado**.

### Véase también

- [Flujo de aprobación](#approval-workflow) para el ciclo de vida completo de una aprobación.
- [Refinar prompts](#refining-prompts) para el flujo "Sigo aprobando el mismo tipo de error".