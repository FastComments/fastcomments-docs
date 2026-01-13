FastComments realiza un seguimiento automático de eventos detallados para cada comentario para proporcionar transparencia en las decisiones de moderación y las acciones del sistema. Estos registros le ayudan a entender por qué un comentario fue aprobado, marcado como spam o tuvo su estado cambiado.

Puede ver los registros de comentarios de comentarios individuales en el panel Moderar comentarios seleccionando un comentario específico.

## Eventos del registro de comentarios

Cada comentario mantiene un registro de los eventos que ocurren durante su ciclo de vida. A continuación se muestran los tipos de eventos que se registran:

### Eventos de anonimización
- **Anonymized** - El contenido del comentario fue borrado y el usuario marcado como eliminado

### Eventos de aprobación
- **ApprovedDueToPastComment** - Comentario aprobado porque el usuario tiene comentarios aprobados anteriormente
- **ApprovedIsAdmin** - Comentario aprobado porque el usuario es administrador
- **NotApprovedRequiresApproval** - El comentario requiere aprobación manual

### Eventos de detección de spam
- **IsSpam** - Comentario marcado como spam por el motor de detección
- **IsSpamDueToBadWords** - Comentario marcado como spam debido al filtro de lenguaje inapropiado
- **IsSpamFromLLM** - Comentario marcado como spam por el motor AI/LLM
- **IsSpamRepeatComment** - Comentario marcado como spam por ser repetitivo
- **NotSpamIsOnlyImage** - Comentario no marcado como spam porque solo contiene imágenes
- **NotSpamIsOnlyReacts** - Comentario no marcado como spam porque solo contiene reacciones
- **NotSpamNoLinkOrMention** - Comentario no marcado como spam debido a que no contiene enlaces o menciones sospechosas
- **NotSpamPerfectTrustFactor** - Comentario no marcado como spam debido a un alto factor de confianza del usuario
- **NotSpamTooShort** - Comentario no marcado como spam porque es demasiado corto para analizar
- **NotSpamSkipped** - La verificación de spam fue omitida
- **NotSpamFromEngine** - Comentario determinado como no spam por el motor de detección

### Eventos de palabras malsonantes/profanidad
- **BadWordsCheckFailed** - La comprobación del filtro de profanidad encontró un error
- **BadWordsFoundBadPhrase** - El filtro de profanidad detectó una frase inapropiada
- **BadWordsFoundBadWord** - El filtro de profanidad detectó una palabra inapropiada
- **BadWordsNoDefinitionForLocale** - No hay definiciones de profanidad disponibles para el idioma del comentario

### Eventos de verificación de usuario
- **CommentMustBeVerifiedToApproveNotInVerifiedSession** - El comentario requiere verificación pero el usuario no está en una sesión verificada
- **CommentMustBeVerifiedToApproveNotVerifiedYet** - El comentario requiere verificación pero el usuario aún no está verificado
- **InVerifiedSession** - El usuario que publica el comentario está en una sesión verificada
- **SentVerificationEmailNoSession** - Se envió un correo de verificación al usuario no verificado
- **SentWelcomeEmail** - Se envió un correo de bienvenida al nuevo usuario

### Eventos de confianza y seguridad
- **TrustFactorChanged** - El factor de confianza del usuario fue modificado
- **SpamFilterDisabledBecauseAdmin** - El filtrado de spam fue omitido para un usuario administrador
- **TenantSpamFilterDisabled** - El filtrado de spam fue desactivado para todo el tenant
- **RepeatCommentCheckIgnored** - La comprobación de comentarios repetidos fue omitida
- **UserIsAdmin** - Usuario identificado como administrador
- **UserIsAdminParentTenant** - Usuario identificado como administrador del parent tenant
- **UserIsAdminViaSSO** - Usuario identificado como administrador vía SSO
- **UserIsMod** - Usuario identificado como moderador

### Cambios de estado del comentario
- **ExpireStatusChanged** - Se modificó el estado de expiración del comentario
- **ReviewStatusChanged** - Se cambió el estado de revisión del comentario
- **SpamStatusChanged** - Se actualizó el estado de spam del comentario
- **ApproveStatusChanged** - Se cambió el estado de aprobación del comentario
- **TextChanged** - Se editó el contenido de texto del comentario
- **VotesChanged** - Se actualizaron los recuentos de votos del comentario
- **Flagged** - El comentario fue marcado por usuarios
- **UnFlagged** - Se eliminaron las marcas del comentario

### Acciones de moderación
- **Pinned** - El comentario fue fijado por un moderador
- **UnPinned** - El comentario fue des-fijado por un moderador
- **RestoredFromAnonymized** - El comentario fue restaurado desde un estado anonimizado

### Eventos de notificación
- **CreatedNotifications** - Se crearon notificaciones para el comentario
- **NotificationCreateFailure** - Falló la creación de notificaciones
- **BadgeAwarded** - Se otorgó una insignia al usuario por el comentario

### Eventos de publicación
- **PublishedLive** - El comentario fue publicado para suscriptores en vivo

### Eventos de integración
- **WebhookSynced** - El comentario fue sincronizado vía webhook

### Eventos de reglas de spam
- **SpamRuleMatch** - El comentario coincidió con una regla de spam personalizada

## Acceso a los registros de comentarios

Los registros de comentarios se generan automáticamente y se almacenan con cada comentario. Proporcionan información valiosa para:

- Comprender las decisiones de moderación
- Solucionar problemas de aprobación o de spam
- Rastrear patrones de comportamiento de los usuarios
- Auditar las acciones del sistema

Estos registros ayudan a mantener la transparencia en el proceso de moderación y a ajustar el comportamiento de su sistema de comentarios.