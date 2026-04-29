Esta es la ruta de cinco minutos desde "tenemos Agentes de IA" hasta "un agente está respondiendo al tráfico en vivo, controlado por aprobaciones." Si quieres la versión larga, cada paso enlaza a la página que lo trata en profundidad.

### 1. Open the AI Agents page

Ve a [Agentes de IA](https://fastcomments.com/auth/my-account/ai-agents) en tu cuenta. La primera vez que llegues aquí verás una de las siguientes opciones:

- Un estado en blanco con un botón **Browse templates** y **Start from scratch** (tienes agentes disponibles para crear), o
- Una página de upsell si tu plan no incluye agentes - ver [Planes y elegibilidad](#plans-and-eligibility).

### 2. Pick a starter template

Haz clic en **Browse templates**. Elige uno de:

- [Moderador](#template-moderator) - revisa comentarios marcados o nuevos, advierte a los primerizos y solo escala a ban después de una advertencia.
- [Saludador de bienvenida](#template-welcome-greeter) - responde a los comentaristas que comentan por primera vez.
- [Fijador de comentarios principales](#template-top-comment-pinner) - fija comentarios sustantivos una vez que superan un umbral de votos.
- [Resumidor de hilos](#template-thread-summarizer) - publica un resumen neutral en hilos largos.

Cada plantilla abre un formulario de edición pre-llenado con **Estado: Ejecución simulada** ya seleccionado.

### 3. Review and save

En el formulario de edición, como mínimo completa:

- **Internal name.** Un identificador corto usado en los paneles de administración.
- **Display name.** Lo que aparece públicamente cuando el agente publica un comentario.
- **Prompt inicial.** Edita el prompt de la plantilla para que coincida con tu tono y tus reglas específicas.
- **Aprobaciones.** Marca las acciones que deberían requerir revisión humana antes de que se ejecuten. Recomendamos al menos `ban_user` para cualquier agente de estilo moderación. Ver [Flujo de aprobaciones](#approval-workflow).

Haz clic en **Save agent**.

### 4. Watch it in dry-run

El agente ya está activo en **Ejecución simulada**. Recibirá sus disparadores, llamará al modelo y registrará acciones en la página de [Historial de ejecuciones](#run-history) - con la insignia **Ejecución simulada** en cada fila - pero no realiza acciones reales. Visita algunos detalles de ejecución (ver [Vista de detalle de ejecución](#run-detail-view)) y observa:

- Las acciones que escogió el agente.
- La justificación y la confianza en cada acción.
- la transcripción completa del LLM.

Si el agente está tomando decisiones con las que no estás de acuerdo, edita el prompt inicial o marca más aprobaciones.

### 5. Run a test against past comments

Desde la página de la lista de agentes, haz clic en **Ejecución de prueba** en la fila del agente. El formulario tiene un único campo numérico **Días** (1 a 90). El tamaño de la muestra y el tope máximo de comentarios evaluados se muestran informativamente: se calculan en el servidor, no los establece el usuario. La reproducción se ejecuta contra comentarios históricos sin tomar acciones reales e informa lo que el agente **habría** hecho frente a lo que realmente ocurrió (si el comentario fue aprobado más tarde, marcado como spam, eliminado, etc.). Ver [Ejecuciones de prueba (Reproducciones)](#test-runs-replays).

### 6. Flip to Enabled

Cuando estés satisfecho con la ejecución simulada y los resultados de la reproducción, edita el agente y cambia **Estado** a **Habilitado**. A partir de aquí, las acciones reales se aplican. La página de Historial de ejecuciones ahora muestra ejecuciones en vivo sin la insignia de ejecución simulada, y cualquier acción que marcaste para aprobación aparece en la [bandeja de entrada de aprobaciones](#approval-workflow).

### Qué sigue

- Configura [Presupuestos](#budgets-overview) y [Alertas de presupuesto](#budget-alerts).
- Configura [Webhooks](#webhooks-overview) si quieres que sistemas externos reaccionen a eventos del agente.
- Añade [Directrices de la comunidad](#community-guidelines) para mantener las decisiones del agente alineadas con tu política escrita.