---
Una **aprobación** es una llamada a una herramienta en cola que requiere que un humano la apruebe o rechace antes de que la plataforma la ejecute.

### Configuración de aprobaciones

En el formulario de edición del agente, la sección **Aprobaciones** enumera cada herramienta que el agente tiene permitido invocar (la lista permitida) y te permite marcar las que deben ser revisadas por un humano. Las herramientas sin marcar se ejecutan inmediatamente. Las marcadas se encolan.

Las herramientas no permitidas son *rechazadas de plano*, no se encolan: las aprobaciones solo se aplican a herramientas que por lo demás están permitidas.

### Qué ocurre cuando se dispara una acción restringida

1. El agente selecciona una llamada a una herramienta (p. ej. `ban_user`) con argumentos, justificación y confianza.
2. En lugar de ejecutarla, la plataforma encola una aprobación en estado `PENDING` con el nombre de la herramienta, los argumentos, la justificación, la confianza y una instantánea del contexto que describe el desencadenante que la produjo.
3. Se envían notificaciones a los revisores (ver [Notificaciones de aprobación](#approval-notifications)).
4. La ejecución del agente se completa y se registra: la acción se muestra como **Pendiente de aprobación** en [Vista de detalle de la ejecución](#run-detail-view).

### Revisión de aprobaciones

La bandeja de aprobaciones en [my-account/ai-agent-approvals](https://fastcomments.com/auth/my-account/ai-agent-approvals) lista las aprobaciones pendientes, aprobadas, rechazadas y con fallo en la ejecución. Para cada una:

- **Nombre de la herramienta y argumentos** - exactamente lo que el agente quiere hacer.
- **Razonamiento del agente** - la justificación suministrada por el agente.
- **Confianza** - la autocalificación de confianza del agente, de 0.0 a 1.0.
- **Instantánea del contexto** - el comentario, la página y el usuario a los que apunta la acción.

Dos botones: **Aprobar** y **Rechazar**. Aprobar ejecuta realmente la herramienta; Rechazar descarta.

### Estados de aprobación

Una aprobación transita por estos estados:

| State | Meaning |
|---|---|
| `PENDING` | A la espera de una decisión humana. |
| `APPROVED` | Un humano aprobó y la acción se ejecutó. |
| `REJECTED` | Un humano rechazó. La acción no se ejecutó. |
| `EXECUTION_FAILED` | Un humano aprobó pero la acción falló (p. ej., el comentario objetivo ya había sido eliminado). |
| `EXECUTING` | Transitorio: un humano hizo clic en Aprobar y la acción se está ejecutando. Se usa para serializar clics de aprobación concurrentes de modo que una herramienta con efectos reales nunca se ejecute dos veces. |

El estado `EXECUTING` importa cuando dos revisores hacen clic en Aprobar simultáneamente: uno gana, el otro ve que la aprobación ya ha cambiado de estado.

### Qué se limpia

Las aprobaciones pendientes permanecen pendientes hasta que se toma una decisión. Expiran automáticamente después de **90 días** desde su creación. Las aprobaciones en cualquier otro estado también se eliminan tras 90 días por higiene de almacenamiento.

Los campos de la aprobación "decidido por" / "decidido en" / "ejecutado en" / "resultado de la ejecución" se rellenan a medida que la aprobación avanza por los estados; todo es visible en la vista de detalle de la bandeja.

### Integración de webhook

Se disparan dos eventos de webhook a medida que las aprobaciones cambian de estado:

- **`approval.requested`** - al insertar en PENDING.
- **`approval.decided`** - al pasar a APPROVED, REJECTED o EXECUTION_FAILED.

Ambos están firmados como cualquier otro webhook. Consulte [Eventos de webhook](#webhook-events) y [Cargas útiles de webhook](#webhook-payloads).

### Fortalecimiento: bloqueo por herramienta conocida

Las aprobaciones no permiten encolar ningún nombre de herramienta que no sea una herramienta reconocida del agente. Esto es una comprobación de defensa en profundidad: incluso si una futura ruta de código pasa un nombre de herramienta derivado de un LLM al flujo de aprobaciones, esa cadena nunca podrá aterrizar como una aprobación en cola. El conjunto de nombres de herramientas conocidas es el mismo que figura en [Referencia de herramientas](#tools-overview).

### Patrones comunes de restricción

- **Agente de moderación nuevo** - restrinja `ban_user`, `mark_comment_spam`, `mark_comment_approved`, `send_email`. Observe la bandeja de entrada durante unas semanas, luego retire la restricción de las herramientas con pocos errores.
- **Agente de moderación a largo plazo** - mantenga `ban_user` y cualquier acción irreversible (`deleteAllUsersComments`, `banIP`) restringidas de forma indefinida.
- **Región UE** - `ban_user` está activado por el Artículo 17 independientemente de lo que marques. Consulte [Cumplimiento del Artículo 17 de la DSA de la UE](#eu-dsa-compliance).

### Lo que las aprobaciones **no** hacen

- No retrasan las otras llamadas a herramientas del agente. La ejecución del agente puede invocar varias herramientas, y solo las restringidas se encolan; el resto se ejecuta con normalidad.
- No revierten la ejecución del agente si el humano rechaza. La parte no restringida de la ejecución ya se ha completado.

---