FastComments aplica el Artículo 17 de la Ley de Servicios Digitales de la UE para inquilinos en la región de la UE: **no se permiten las suspensiones de usuarios totalmente automatizadas**.

### Lo que eso significa en la práctica

Cuando tu inquilino está en la región de la UE, en el formulario de edición del agente:

- La casilla **Aprobaciones** para `ban_user` está **bloqueada activada** y no se puede desmarcar.
- La etiqueta dice: "EU DSA Article 17: user suspensions require human review. 'Ban a user' is locked on and cannot be fully automated in the EU region."
- Una descripción emergente en la columna de aprobaciones dice: "Locked on by EU DSA Article 17 - fully-automated bans are not permitted in the EU region."

Pase lo que pase con el resto de tu configuración, cada llamada a `ban_user` de cualquier agente en un inquilino en la región de la UE va a la [bandeja de aprobaciones](#approval-workflow) para revisión humana. La suspensión no ocurre hasta que un humano la apruebe.

### Por qué esto se aplica a nivel de plataforma y no a nivel de prompt

Los system prompts pueden ser ignorados o eludidos por un modelo que se comporte mal. El cumplimiento del Artículo 17 es demasiado importante para confiar en el buen comportamiento del modelo; tiene que ser una puerta rígida del lado del servidor que el propio despachador de herramientas haga cumplir. Y eso es lo que hacemos.

### Qué sí y qué no pasa por aprobación

- **`ban_user`**: siempre está bloqueado en la UE. Incluyendo:
  - Suspensiones visibles (`shadowBan: false`).
  - Shadow bans (`shadowBan: true`).
  - Suspensiones con `deleteAllUsersComments: true`.
  - Suspensiones con `banIP: true`.
- Todas las variantes de suspensión llegan a la bandeja de aprobaciones con el razonamiento y la confianza del agente; un revisor humano aprueba o rechaza.

Las otras herramientas del agente (`mark_comment_spam`, `warn_user`, `lock_comment`, etc.) **no** se ven afectadas por el Artículo 17. Aún puedes automatizarlas. El Artículo 17 se refiere específicamente a las suspensiones de usuarios.

### Qué pasa con inquilinos fuera de la UE

El bloqueo no se aplica fuera de la región de la UE. Puedes elegir someter `ban_user` a aprobación de todos modos: lo recomendamos encarecidamente durante las primeras semanas de vida de cualquier agente de moderación, pero no es obligatorio.

### Shadow bans

Los shadow bans cuentan como suspensiones a efectos del Artículo 17 (el usuario puede publicar pero su contenido está oculto). Están restringidos de la misma manera que las suspensiones visibles.

### Detección de la región

La región se determina a nivel de proceso mediante la variable de entorno `REGION` en la implementación de FastComments (leída por `isEURegion()` en `models/constants.ts`). No hay un campo de región por inquilino: el bloqueo se aplica a todos los inquilinos en una instancia desplegada en la UE. Si migras tus datos desde un despliegue fuera de la UE a uno en la UE, el bloqueo entra en vigor para todos los inquilinos de esa instancia.

### Qué ocurre si todos los revisores no están disponibles

La aprobación permanecerá en la bandeja de entrada hasta que se decida. Expira automáticamente 90 días después de su creación. No existe una vía "sin revisores disponibles, pasar a decisión automatizada" — eso iría en contra del propósito del Artículo 17.

Si tu comunidad tiene tanto volumen que las suspensiones en la UE no pueden ser revisadas en un tiempo razonable, considera:

- Añadir más revisores (ver [Approval Notifications](#approval-notifications)).
- Cambiar el agente para usar [`warn_user`](#tool-warn-user) de forma más agresiva, ya que las advertencias no están sujetas al Artículo 17.
- Reducir la predisposición del agente a suspender ajustando las [directrices de la comunidad](#community-guidelines) o el [prompt inicial](#personality-prompt).

### Véase también

- [Herramienta: ban_user](#tool-ban-user) para lo que hace `ban_user` y las opciones destructivas tras opt-ins adicionales.
- [Approval Workflow](#approval-workflow) para el ciclo de vida completo de la aprobación.