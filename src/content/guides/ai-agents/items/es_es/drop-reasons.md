Cuando un trigger se activa para un agente pero **no** resulta en una llamada a un LLM, la plataforma registra un "drop" con una razón. Los drops aparecen en la [Página de Analytics](#analytics-page) bajo "Disparadores omitidos (este mes)".

### La lista completa de razones de drop

| Razón | Qué sucedió |
|---|---|
| `agentDaily` | Se alcanzó el límite diario del presupuesto del agente. |
| `agentMonthly` | Se alcanzó el límite mensual del presupuesto del agente. |
| `tenantDaily` | Se alcanzó el límite diario del presupuesto del tenant. |
| `tenantMonthly` | Se alcanzó el límite mensual del presupuesto del tenant. |
| `qps` | Se alcanzó el límite de tasa por minuto del agente (ventana rodante de 60 s). |
| `concurrency` | Se saturó el número máximo de ejecuciones concurrentes del agente. |

### Qué no está en esta lista

Un trigger que nunca llega al path de dispatch no se considera "dropped" con una razón: simplemente no se despacha. Esto incluye:

- El agente está **Deshabilitado**.
- El comentario que dispara no coincide con el [alcance URL/locale](#scope-url-locale) del agente.
- La acción que dispara fue realizada por el mismo agente (prevención de bucles).
- El tenant tiene facturación inválida.
- El agente no está en el plan del tenant.

Estos son saltos silenciosos, no drops. No aparecen en el gráfico de drops en Analytics.

### Leer drops en Analytics

La [Página de Analytics](#analytics-page) muestra:

- **Disparadores omitidos (este mes)** - recuentos agrupados por razón de drop.
- **Agentes en o cerca de su límite** - desglose por agente de qué agentes están alcanzando el límite, con un conteo de triggers descartados en el periodo actual.

### Qué hacer cuando ves drops

- **`agentDaily` / `agentMonthly`** - el tope propio del agente es demasiado restrictivo. O aumenta el tope en el formulario de edición o reduce el alcance del agente (URL/locale, triggers más específicos).
- **`tenantDaily` / `tenantMonthly`** - el tope a nivel de cuenta es demasiado restrictivo. Auméntalo en la configuración de facturación del tenant, o distribuye el gasto entre menos agentes.
- **`qps`** - el tráfico está alcanzando el límite por minuto de la ventana rodante. A menudo es signo de un hilo viral que dispara triggers más rápido de lo que el agente puede ejecutarlos. Los campos `maxTriggersPerMinute` y `maxConcurrent` del agente limitan esto; aumentarlos incrementa el rendimiento pero también incrementa el coste por picos.
- **`concurrency`** - misma causa raíz que `qps` pero en el recuento de ejecuciones en curso. Aumenta `maxConcurrent` si necesitas más paralelismo.

### Drops vs errores

Un drop significa "el trigger nunca se ejecutó". Un **error** significa "el trigger se ejecutó pero la llamada al LLM o el dispatch de la herramienta falló". Los errores se rastrean por separado en la página de [Run History](#run-history) (estado `Error`).

### Los drops también pueden detener replays

Las mismas razones de drop detienen las [ejecuciones de prueba / replays](#test-runs-replays) en curso. El replay se detiene con un estado Error y un mensaje que indica qué presupuesto se alcanzó (por ejemplo, el presupuesto diario del agente).

### La prevención de bucles es silenciosa a propósito

No existe una razón de drop para "este trigger provino de otro agente y fue omitido para evitar un bucle". Registrarlo llenaría la analítica sin aportar señal útil: por diseño, el fan-out de agentes nunca debería resultar en una explosión de triggers. Si sospechas que se está suprimiendo un bucle donde no debería, verifica los [Comment Logs](/guide-moderation.html#comment-logs) - el `botId` en los comentarios generados por bots es lo que utiliza la comprobación de bucles.