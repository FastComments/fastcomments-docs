Each agent runs against one of two LLM models, picked on the **Model** section of the edit form.

### Las dos opciones

- **GLM 5.1 (DeepInfra) - Más inteligente, un poco más lento** - el predeterminado. Mayor calidad de razonamiento, algo más lento por llamada. Recomendado para agentes de estilo moderación (plantilla `Moderator`, cualquier cosa que invoque `ban_user` o `mark_comment_spam`) donde el coste de una llamada equivocada es alto.

- **GPT-OSS 120B Turbo (DeepInfra) - Más rápido** - más rápido por llamada, menor latencia. Recomendado para agentes de alto volumen y bajo riesgo (saludador de bienvenida, fijador de hilos) donde quieres respuestas en segundos y las consecuencias de una llamada incorrecta son menores.

Ambos modelos soportan llamadas a funciones, ambos se ejecutan a través de la misma API compatible con OpenAI, y ambos comparten los mismos esquemas por herramienta, por lo que puedes cambiar un agente guardado entre ellos en cualquier momento sin otros cambios de configuración.

### Diferencias de coste

Los dos modelos tienen diferentes costes por token. Los [límites de presupuesto](#budgets-overview) del agente están denominados en la moneda de tu cuenta, no en tokens, así que un cambio de un modelo a otro altera cuántas ejecuciones caben dentro de tus límites diarios y mensuales. La página de [Historial de ejecuciones](#run-history) muestra el coste por ejecución en tu moneda una vez que una ejecución termina: ver unas cuantas ejecuciones tras un cambio es la forma más sencilla de evaluar la nueva tasa de consumo.

### Tokens por ejecución

El uso de tokens de respuesta del modelo se registra en cada activación como **tokensUsed**. Se incluye en las cargas útiles webhook `trigger.succeeded` y `trigger.failed` (ver [Cargas útiles de webhook](#webhook-payloads)) y se muestra en la [Vista de detalle de ejecución](#run-detail-view). La cantidad depende de:

- Cuánto [Contexto](#context-options) incluyas: el contexto del hilo, el historial del usuario y los metadatos de la página añaden tokens.
- La longitud de tu [Mensaje inicial](#personality-prompt) y de las [Directrices de la comunidad](#community-guidelines).
- Cuántas herramientas llama el agente en una sola ejecución (cada llamada a herramienta y su resultado hacen un viaje de ida y vuelta a través del modelo).

**Máx. Tokens Por Activación** (por defecto 20,000) es el límite superior por ejecución, establecido por agente.

### Cambiar de modelo

Puedes cambiar modelos en el formulario de edición en cualquier momento. El historial de ejecuciones y las analíticas existentes mantienen sus números originales de tokens y costes: se registran en tiempo de ejecución. El nuevo modelo solo se aplica a las ejecuciones que empiezan después de que guardes.

No existe una opción de "usar el modelo que sea más barato". La elección es explícita por agente.