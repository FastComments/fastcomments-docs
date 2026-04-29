---
El costo del agente está **basado en tokens**. Cada llamada al LLM devuelve un recuento de tokens, la plataforma lo convierte a centavos USD usando la tarifa por token del modelo, y los centavos se cargan contra los presupuestos del agente y del tenant.

### Qué se factura

- **Todas las llamadas al LLM**, incluyendo la llamada que no produce acciones de herramientas ("el agente decidió no hacer nada"). La inferencia se paga incluso cuando no resulta ninguna acción.
- **Llamadas Dry-run**. Dry-run es "no actuar, pero aún así llamar al LLM" - la llamada al LLM cuesta lo mismo. Véase [Modo Dry-Run](#dry-run-mode).
- **Llamadas de reproducción (Replays)**. Las reproducciones son ejecuciones Dry-run contra comentarios históricos. Generan consumo de tokens. Véase [Ejecuciones de prueba (Replays)](#test-runs-replays).

### Qué no se factura

- **Disparadores que nunca producen una llamada al LLM.** Los casos descartados antes de la llamada al LLM (exceso de presupuesto, limitados por tasa, incompatibilidad de alcance, facturación inválida, prevención de bucles) no cuestan tokens. Véase [Razones de descarte](#drop-reasons).
- **Despacho de herramientas.** Llamar a `pin_comment` u otra herramienta no cuesta tokens por sí mismo: solo la ida y vuelta al LLM tiene costo.
- **`search_memory`.** Es de solo lectura y no produce su propia ida y vuelta al LLM.

### Costo por ejecución

Una única ejecución de agente puede llamar al LLM varias veces - cada resultado de llamadas a herramientas se reinyecta en el modelo para que pueda llamar a otra herramienta o finalizar. Por tanto, `tokensUsed` en una ejecución es la suma de todas las idas y vueltas al LLM en esa ejecución.

Los mayores contribuyentes al costo en tokens por ejecución:

- **Prompts iniciales largos ([prompts iniciales](#personality-prompt)) y [directrices de la comunidad](#community-guidelines)** - se incluyen en cada ejecución.
- **[Opciones de contexto](#context-options)** - contexto del hilo, historial de usuario, metadatos de la página. Cada uno añade tokens.
- **El texto del comentario en sí** - los comentarios largos cuestan más.
- **Múltiples llamadas a herramientas en una misma ejecución** - el mensaje de resultado de cada herramienta se envía de nuevo al modelo.
- **Lecturas de memoria** - `search_memory` devuelve hasta 25 registros (limitado a 8000 caracteres de contenido total). La mayoría de esos bytes se incluyen en el siguiente prompt.

**Max Tokens Per Trigger** (default 20,000) limita el tamaño de la **respuesta** por llamada al LLM. No limita el tamaño de la entrada.

### Conversión de tokens a centavos

La plataforma aplica una única tarifa por paquete de tenant (`flexLLMCostCents` por `flexLLMUnit` tokens). El costo por token es a nivel de paquete, no por modelo - ambos modelos disponibles ([GLM 5.1 and GPT-OSS Turbo](#choosing-a-model)) se facturan a la misma tarifa en un paquete dado. La [Vista de detalles de la ejecución](#run-detail-view) muestra el costo por ejecución en su moneda una vez que la ejecución finaliza.

### Dónde se registra el costo

Cada ejecución registra su recuento bruto de tokens y el costo por ejecución. Los totales diarios y mensuales se consolidan en la [página de Analíticas](#analytics-page).

### Cómo interpretar el costo

- **Costo por ejecución**: [Vista de detalles de la ejecución](#run-detail-view) -> campo `Cost`.
- **Agregado diario/mensual**: [página de Analíticas](#analytics-page) -> gráficos de uso del presupuesto y costo diario.
- **Costo por acción**: también en la Vista de detalles de la ejecución, útil para ajustar cuando el bucle de herramientas de un agente es inusualmente largo.

### Véase también

- [Elegir un modelo](#choosing-a-model) - el principal factor en el costo.
- [Opciones de contexto](#context-options) - de dónde proviene el costo adicional.
- [Resumen de presupuestos](#budgets-overview) - topes estrictos que previenen costos descontrolados.

---