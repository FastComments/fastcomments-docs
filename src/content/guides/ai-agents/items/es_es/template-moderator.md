**ID de plantilla:** `tos_enforcer`

La plantilla Moderator es el punto de partida recomendado si tu objetivo es reducir la carga de moderación manual. Revisa comentarios nuevos y señalados y aplica las normas de tu comunidad.

Casi siempre querrás **aumentar el prompt integrado** con ejemplos concretos de lo que tu sitio permite y no permite. La propia política de escalado de la plataforma (advertir antes de prohibir, buscar en la memoria antes de prohibir) ya está integrada en el prompt del sistema que recibe el agente, así que no necesitas repetirla.

### Disparadores

- **New comment posted** (`COMMENT_ADD`) - el agente revisa cada comentario nuevo.
- **Comment crosses a flag threshold** (`COMMENT_FLAG_THRESHOLD`, default threshold: 3) - el agente reevalúa un comentario que otros usuarios han señalizado.

### Herramientas permitidas

- [`mark_comment_approved`](#tools-overview) - útil para inquilinos con pre-moderación donde el agente publica los comentarios limpios y oculta el resto.
- [`mark_comment_spam`](#tools-overview)
- [`warn_user`](#tool-warn-user)
- [`ban_user`](#tool-ban-user)

No puede publicar comentarios, votar, fijar, bloquear, otorgar insignias ni enviar correos electrónicos: el prompt es intencionalmente limitado.

### Añadidos recomendados antes de lanzarlo en producción

- **Establece las [Directrices de la comunidad](#community-guidelines).** Unas pocas frases de política escrita son suficientes; el agente las aplica en cada ejecución.
- **Restringe `ban_user` mediante [aprobación](#approval-workflow).** Esto está activado por defecto en la región de la UE (ver [Cumplimiento del artículo 17 de la DSA de la UE](#eu-dsa-compliance)) y se recomienda en todas partes.
- **Considera también restringir `mark_comment_spam` mediante aprobación** si tienes contenido de bajo volumen pero de alto impacto.
- **Restringe `mark_comment_approved` mediante aprobación si ejecutas pre-moderación.** Aprobar un comentario malo lo pone frente a los lectores; resérvalo hasta que el agente haya ganado confianza mediante el modo de prueba.
- **Marca "Incluir el factor de confianza del comentarista, la antigüedad de la cuenta, el historial de prohibiciones y los comentarios recientes"** en [Opciones de contexto](#context-options). El modelo advertirá con mucha menos agresividad cuando pueda ver que alguien es un usuario de buena fe y de larga trayectoria.

### Ventana de modo de prueba recomendada

Ejecuta esta plantilla en [modo de prueba](#dry-run-mode) durante al menos una semana contra tu tráfico real antes de cambiar a Habilitado. Usa [Ejecuciones de prueba (Repeticiones)](#test-runs-replays) para previsualizar también contra los 30 días anteriores.