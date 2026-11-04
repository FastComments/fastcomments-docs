Cada agente se ejecuta sobre uno de dos modelos LLM, elegido en la sección **Model** del formulario de edición.

### Las dos opciones

- **GLM 5.1 (DeepInfra) - Más inteligente, algo más lento** - el predeterminado. Mayor calidad de razonamiento, algo más lento por llamada. Recomendado para agentes de tipo moderación (plantilla `Moderator`, cualquier cosa que llame a `ban_user` o `mark_comment_spam`) donde el costo de una llamada incorrecta es alto.

- **GPT-OSS 120B Turbo (DeepInfra) - Más rápido** - más rápido por llamada, menor latencia. Recomendado para agentes de alto volumen y bajo riesgo (saludo de bienvenida, fijado de hilos) donde quieres respuestas en segundos y las consecuencias de una llamada equivocada son menores.

Ambos modelos soportan function calling, ambos se ejecutan a través de la misma API compatible con OpenAI, y ambos comparten los mismos esquemas por herramienta - por lo que puedes cambiar un agente guardado entre ellos en cualquier momento sin otros cambios de configuración.

### Diferencias de costo

Los dos modelos tienen costos por token diferentes. Los [límites de presupuesto](#budgets-overview) del agente se expresan en la moneda de tu cuenta, no en tokens, por lo que cambiar de un modelo a otro cambia cuántas ejecuciones caben dentro de tus límites diarios y mensuales. La página [Run History](#run-history) muestra el costo por ejecución en tu moneda una vez que la ejecución se completa - observar algunas ejecuciones después de un cambio es la forma más fácil de medir la nueva tasa de consumo.

### Tokens por ejecución

El uso de tokens de respuesta del modelo se registra en cada activación como **tokensUsed**. Se incluye en las cargas de los webhooks `trigger.succeeded` y `trigger.failed` (ver [Webhook Payloads](#webhook-payloads)) y se muestra en [Run Detail View](#run-detail-view). La cantidad depende de:

- Cuánto [Context](#context-options) incluyas - el contexto del hilo, el historial del usuario y los metadatos de la página añaden tokens.
- Qué tan largos sean tu [Initial prompt](#personality-prompt) y tus [Community guidelines](#community-guidelines).
- Cuántas herramientas llama el agente en una sola ejecución (cada llamada a herramienta y su resultado hace un viaje de ida y vuelta a través del modelo).

**Max Tokens Per Trigger** (predeterminado 20,000) es el límite superior por ejecución, configurado por agente.

### Cambiar de modelo

Puedes cambiar de modelo en el formulario de edición en cualquier momento. El historial de ejecuciones existente y las analíticas conservan sus números originales de tokens y costos - se registran en el momento de la ejecución. El nuevo modelo solo se aplica a las ejecuciones que comienzan después de guardar.

No existe una opción de "usar el modelo que sea más barato". La elección es explícita por agente.