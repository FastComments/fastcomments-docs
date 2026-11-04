FastComments aplica el Artículo 17 de la Ley de Servicios Digitales de la UE para los tenants en la región de la UE: **no se permiten suspensiones de usuarios totalmente automatizadas**.

### Qué significa esto en la práctica

Cuando tu tenant está en la región de la UE, en el formulario de edición del agente:

- La casilla de **Approvals** para `ban_user` está **bloqueada en activado** y no se puede desmarcar.
- La etiqueta dice: "EU DSA Article 17: las suspensiones de usuarios requieren revisión humana. 'Ban a user' está bloqueado en activado y no puede automatizarse por completo en la región de la UE."
- Un tooltip en la columna de aprobaciones dice: "Bloqueado en activado por el Artículo 17 de la Ley de Servicios Digitales de la UE - no se permiten baneos totalmente automatizados en la región de la UE."

Independientemente de lo demás que configures, cada llamada a `ban_user` de cualquier agente en un tenant de la región de la UE va al [buzón de aprobaciones](#approval-workflow) para revisión humana. El baneo no ocurre hasta que un humano lo aprueba.

### Por qué se aplica a nivel de plataforma, no a nivel de prompt

Los prompts del sistema pueden ser ignorados o eludidos por un modelo que se comporte lo suficientemente mal. El cumplimiento del Artículo 17 es demasiado importante para depender del buen comportamiento del modelo; tiene que ser una barrera estricta del lado del servidor que el propio despachador de herramientas aplique. Que es exactamente lo que hacemos.

### Qué pasa y qué no pasa por aprobación

- **`ban_user`**: siempre sujeto a aprobación en la UE. Incluyendo:
  - Baneos visibles (`shadowBan: false`).
  - Shadow bans (`shadowBan: true`).
  - Baneos con `deleteAllUsersComments: true`.
  - Baneos con `banIP: true`.
- Todas las variantes de baneo llegan al buzón de aprobaciones con el razonamiento y la confianza del agente; un humano aprueba o rechaza.

Las demás herramientas del agente (`mark_comment_spam`, `warn_user`, `lock_comment`, etc.) **no** se ven afectadas por el Artículo 17. Puedes seguir automatizándolas. El Artículo 17 se refiere específicamente a las suspensiones de usuarios.

### ¿Qué pasa con los tenants fuera de la UE?

El bloqueo no se aplica fuera de la región de la UE. Puedes elegir poner `ban_user` detrás de una aprobación de todos modos - lo recomendamos encarecidamente durante las primeras semanas de vida de cualquier agente de moderación - pero no es obligatorio.

### Shadow bans

Los shadow bans cuentan como suspensiones a efectos del Artículo 17 (el usuario puede publicar pero su contenido está oculto). Se someten a aprobación de la misma forma que los baneos visibles.

### Detección de región

La región se determina a nivel de proceso mediante la variable de entorno `REGION` en el despliegue de FastComments (leída por `isEURegion()` en `models/constants.ts`). No hay un campo de región por tenant - el bloqueo se aplica a todos los tenants de una instancia desplegada en la UE. Si migras tus datos de un despliegue fuera de la UE a un despliegue en la UE, el bloqueo entra en vigor para todos los tenants de esa instancia.

### ¿Qué pasa si no hay revisores disponibles?

La aprobación permanecerá en el buzón hasta que se decida. Caduca automáticamente 90 días después de su creación. No existe una vía de "no hay revisor disponible, pasar a una decisión automatizada" - eso anularía el propósito del Artículo 17.

Si tu comunidad tiene un volumen tan alto que los baneos en la UE no pueden revisarse en un tiempo razonable, considera:

- Añadir más revisores (ver [Notificaciones de aprobación](#approval-notifications)).
- Cambiar el agente para usar [`warn_user`](#tool-warn-user) con más frecuencia, ya que las advertencias no están sujetas al Artículo 17.
- Reducir la propensión del agente a banear endureciendo las [normas de la comunidad](#community-guidelines) o el [prompt inicial](#personality-prompt).

### Ver también

- [Herramienta: ban_user](#tool-ban-user) para saber qué hace `ban_user` y las opciones destructivas que requieren consentimientos adicionales.
- [Flujo de aprobación](#approval-workflow) para el ciclo de vida completo de las aprobaciones.