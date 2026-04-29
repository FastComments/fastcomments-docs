**Template ID:** `tos_enforcer`

La plantilla Moderator es el punto de partida recomendado si tu objetivo es reducir la carga de moderación manual. Revisa los comentarios nuevos y señalados y aplica las reglas de tu comunidad.

### Prompt inicial integrado

[inline-code-attrs-start title = 'Prompt inicial de la plantilla Moderator'; type='text' inline-code-attrs-end]
[inline-code-start]
You are a terms-of-service enforcement agent. Review each new comment against the community guidelines. Mark clear spam or policy violations. Issue warnings for first-offense borderline content. Escalate ban decisions only for repeat or severe violations. If a comment is clearly within the rules, approve it so it becomes visible (relevant for pre-moderation tenants). Stay out of political or subjective debates, focus on the rules as written.
[inline-code-end]

Casi siempre querrás **aumentar este prompt** con ejemplos concretos de lo que tu sitio permite y no permite. La propia política de escalada de la plataforma (advertir antes de banear, buscar en la memoria antes de banear) ya está integrada en el prompt del sistema que recibe el agente, así que no necesitas repetirla.

### Disparadores

- **New comment posted** (`COMMENT_ADD`) - el agente examina cada comentario nuevo.
- **Comment crosses a flag threshold** (`COMMENT_FLAG_THRESHOLD`, default threshold: 3) - el agente reevalúa un comentario que otros usuarios han marcado.

### Herramientas permitidas

- [`mark_comment_approved`](#tools-overview) - útil para inquilinos con pre-moderación donde el agente publica los comentarios limpios y oculta el resto.
- [`mark_comment_spam`](#tools-overview)
- [`warn_user`](#tool-warn-user)
- [`ban_user`](#tool-ban-user)

No puede publicar comentarios, votar, fijar, bloquear, otorgar insignias ni enviar correos electrónicos: el prompt está intencionalmente limitado.

### Recomendaciones antes de ir en vivo

- **Establece las [Directrices de la comunidad](#community-guidelines).** Unas pocas frases de política escrita son suficientes; el agente las aplica en cada ejecución.
- **Restringe `ban_user` detrás de [aprobación](#approval-workflow).** Esto está activado por defecto en la región de la UE (ver [EU DSA Article 17 Compliance](#eu-dsa-compliance)) y se recomienda en todas partes.
- **Considera también restringir `mark_comment_spam` detrás de aprobación** si tienes contenido de bajo volumen pero de alto riesgo.
- **Restringe `mark_comment_approved` detrás de aprobación si ejecutas pre-moderación.** Aprobar un mal comentario lo pone frente a los lectores; restríngelo hasta que el agente haya demostrado fiabilidad mediante ejecuciones de prueba.
- **Marca la opción "Include commenter's trust factor, account age, ban history, and recent comments"** en las [Context Options](#context-options). El modelo advertirá con mucha menos agresividad cuando pueda ver que alguien es un usuario de buena fe de larga trayectoria.

### Ventana recomendada para dry-run

Ejecuta esta plantilla en [dry-run](#dry-run-mode) durante al menos una semana contra tu tráfico real antes de cambiar a Enabled. Usa [Test Runs (Replays)](#test-runs-replays) para previsualizar también contra los últimos 30 días.

---