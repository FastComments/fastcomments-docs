Esta es la ruta de cinco minutos desde "tenemos Agentes de IA" hasta "un agente está respondiendo a tráfico real, controlado por aprobaciones". Si quieres la versión extensa, cada paso enlaza a la página que lo cubre en profundidad.

### 1. Abre la página de Agentes de IA

Ve a [AI Agents](https://fastcomments.com/auth/my-account/ai-agents) en tu cuenta. La primera vez que llegues aquí verás una de estas opciones:

- Una pantalla en blanco con los botones **Browse templates** y **Start from scratch** (tienes agentes disponibles para crear), o
- Una página de venta adicional si tu plan no incluye agentes - consulta [Planes y Elegibilidad](#plans-and-eligibility).

### 2. Elige una plantilla inicial

Haz clic en **Browse templates**. Elige una de:

- [Moderator](#template-moderator) - revisa comentarios marcados o nuevos, advierte a los primerizos y escala a un ban solo después de una advertencia.
- [Welcome Greeter](#template-welcome-greeter) - responde a los comentaristas primerizos.
- [Top Comment Pinner](#template-top-comment-pinner) - fija comentarios sustanciales una vez que superan un umbral de votos.
- [Thread Summarizer](#template-thread-summarizer) - publica un resumen neutro en hilos largos.

Cada plantilla aterriza en un formulario de edición prellenado con **Status: Dry Run** ya seleccionado.

### 3. Revisa y guarda

En el formulario de edición, haz como mínimo lo siguiente:

- **Nombre interno.** Un identificador corto usado en los paneles de administración.
- **Nombre para mostrar.** Lo que aparece públicamente cuando el agente publica un comentario.
- **Prompt inicial.** Edita el prompt de la plantilla para que coincida con tu tono y tus reglas específicas.
- **Aprobaciones.** Marca las acciones que deberían requerir revisión humana antes de surtir efecto. Recomendamos al menos `ban_user` para cualquier agente de tipo moderación. Consulta [Flujo de Aprobación](#approval-workflow).

Haz clic en **Save agent**.

### 4. Obsérvalo en modo dry-run

El agente ahora está activo en **Dry Run**. Recibirá sus disparadores, llamará al modelo y registrará acciones en la página [Run History](#run-history) - con la insignia **Dry Run** en cada fila - pero no tomará acciones reales. Visita algunos de los detalles de ejecución (ver [Vista de Detalle de Ejecución](#run-detail-view)) y revisa:

- Las acciones que el agente eligió.
- La justificación y la confianza de cada acción.
- La transcripción completa del LLM.

Si el agente está tomando decisiones con las que no estás de acuerdo, edita el prompt inicial o marca más aprobaciones.

### 5. Ejecuta una prueba contra comentarios pasados

Desde la página de lista de agentes, haz clic en **Test run** en la fila del agente. El formulario tiene una única entrada numérica **Days** (1 a 90). El tamaño de la muestra y el límite máximo de comentarios evaluados se muestran de forma informativa - se calculan en el servidor, no los establece el usuario. La repetición se ejecuta contra comentarios históricos sin tomar acciones reales e informa lo que el agente **habría** hecho frente a lo que realmente ocurrió (si el comentario fue aprobado después, marcado como spam, eliminado, etc.). Consulta [Ejecuciones de Prueba (Replays)](#test-runs-replays).

### 6. Cambia a Enabled

Cuando estés satisfecho con los resultados del dry-run y del replay, edita el agente y cambia **Status** a **Enabled**. A partir de aquí, las acciones reales se ejecutan. La página Run History ahora muestra ejecuciones en vivo sin la insignia de dry-run, y cualquier acción que hayas marcado para aprobación aparece en la [bandeja de aprobaciones](#approval-workflow).

### Qué sigue

- Configura [Presupuestos](#budgets-overview) y [Alertas de Presupuesto](#budget-alerts).
- Configura [Webhooks](#webhooks-overview) si quieres que sistemas externos reaccionen a los eventos de los agentes.
- Añade [Directrices de la Comunidad](#community-guidelines) para mantener las decisiones del agente alineadas con tu política escrita.