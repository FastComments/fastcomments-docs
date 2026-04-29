**Modo de prueba** es el modo de seguridad en el que empieza todo agente nuevo. El agente se ejecuta de extremo a extremo excepto en la parte donde interactúa con tu comunidad.

### Qué se ejecuta en Modo de prueba

- Los triggers se activan normalmente.
- Se ensamblan el prompt del agente, las [directrices de la comunidad](#community-guidelines) y el [contexto](#context-options).
- Se llama al LLM.
- El modelo selecciona llamadas a herramientas y proporciona justificaciones + puntuaciones de confianza.
- La ejecución se registra con una insignia **Modo de prueba** para que se distinga claramente de las ejecuciones en vivo.

### Qué no se ejecuta en Modo de prueba

- No se publica ningún comentario, no se emite ningún voto, no se fija/desfija/bloquea/desbloquea ningún comentario.
- No se marca ningún comentario como spam, aprobado o revisado.
- No se banea, advierte ni concede insignias a ningún usuario.
- No se envía ningún correo electrónico.
- No se escribe memoria. (Sí: incluida la memoria. Los agentes en modo de prueba no pueden envenenar la memoria compartida con decisiones hipotéticas.)
- No se disparan webhooks por acciones de herramientas. (Los webhooks a nivel de trigger `trigger.succeeded` / `trigger.failed` sí se desencadenan y la carga útil incluye `wasDryRun: true`. Ver [Webhook Payloads](#webhook-payloads).)

### Qué cuesta

Las ejecuciones en Modo de prueba realizan **la misma llamada al LLM** que haría una ejecución habilitada. Se cobran tokens, se aplican los [límites de presupuesto](#budgets-overview) y las ejecuciones cuentan para los límites diarios/mensuales por agente y por tenant.

Ese costo es el precio de obtener una vista previa fiel. Un modo de “saltar la llamada al LLM” no te daría ninguna señal sobre cómo se comportaría el agente.

### Leer los resultados de modo de prueba

En el [Historial de ejecuciones](#run-history), las ejecuciones en modo de prueba aparecen señaladas con la insignia **Modo de prueba** en la columna de estado. Las acciones dentro de cada ejecución parecen idénticas a las acciones en vivo: mismo nombre de herramienta, mismos argumentos, misma justificación y confianza; excepto que ninguna ocurrió realmente.

La [página de análisis](#analytics-page) desglosa por mes las ejecuciones "modo de prueba vs en vivo" para que puedas ver cuánto de tu gasto en tokens fue destinado a observación.

### Salir del Modo de prueba

Edita el agente y cambia **Status** a **Enabled**. El siguiente trigger se ejecutará en vivo.

También puedes cambiar en sentido contrario — de Enabled a Modo de prueba — si el agente empieza a hacer cosas que no te gustan. No hay penalización.

### Las repeticiones obligan el Modo de prueba

La función [Ejecuciones de prueba (Repeticiones)](#test-runs-replays) ejecuta el agente contra comentarios históricos **siempre en modo de prueba**, independientemente del estado guardado del agente. Las repeticiones no pueden realizar acciones reales sobre comentarios pasados. Esto es por diseño: la repetición es una herramienta de vista previa, no una herramienta de moderación.