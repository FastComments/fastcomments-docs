FastComments realiza un seguimiento automático de eventos detallados para cada comentario para proporcionar transparencia en las decisiones de moderación y las acciones del sistema. Estos registros te ayudan a entender por qué un comentario fue aprobado, marcado como spam o tuvo su estado cambiado.

## Accessing Comment Logs

Para ver los registros de un comentario específico:

1. Navega a la página **Moderar comentarios** en el panel de control de FastComments
2. Encuentra el comentario que deseas inspeccionar
3. Haz clic en el botón **Ver registros** (icono de reloj) en la barra de acciones del comentario
4. Aparecerá un cuadro de diálogo que muestra el historial completo de eventos para ese comentario

Cada entrada del registro muestra:
- **When** - La marca de tiempo del evento
- **Who** - El usuario o sistema que desencadenó el evento (cuando aplica)
- **What** - El tipo de acción o evento
- **Details** - Contexto adicional como valores antes/después, nombres de motores o datos relacionados

## Comment Log Events

Cada comentario mantiene un registro de eventos que ocurren durante su ciclo de vida. A continuación se muestran los tipos de eventos que se registran:

### Anonymization Events
- **Anonymized** - El contenido del comentario fue borrado y el usuario marcado como eliminado
- **RestoredFromAnonymized** - El comentario fue restaurado desde el estado anonimizado

### Approval Events
- **ApprovedDueToPastComment** - Comentario aprobado porque el usuario tiene comentarios previos aprobados (incluye referencia al comentario anterior)
- **ApprovedIsAdmin** - Comentario aprobado porque el usuario es administrador
- **NotApprovedRequiresApproval** - El comentario requiere aprobación manual
- **NotApprovedLowTrustFactor** - Comentario no aprobado debido a un bajo factor de confianza del usuario (incluye el valor del factor de confianza)

### Profile Comment Approval Events

Estos eventos se aplican específicamente a comentarios en perfiles de usuario:

- **ApprovedProfileAutoApproveAll** - Comentario en perfil aprobado automáticamente porque el propietario del perfil ha activado la autoaprobación para todos los comentarios
- **ApprovedProfileTrusted** - Comentario en perfil aprobado porque el comentarista es de confianza (incluye referencia al comentario que estableció la confianza)
- **NotApprovedProfileManualApproveAll** - Comentario en perfil requiere aprobación manual porque el propietario del perfil ha activado la aprobación manual
- **NotApprovedProfileNotTrusted** - Comentario en perfil no aprobado porque el comentarista no es de confianza
- **NotApprovedProfileNewUser** - Comentario en perfil no aprobado porque el comentarista es un usuario nuevo

### Spam Detection Events
- **IsSpam** - Comentario marcado como spam por el motor de detección (incluye qué motor tomó la decisión)
- **IsSpamDueToBadWords** - Comentario marcado como spam debido al filtro de profanidad
- **IsSpamFromLLM** - Comentario marcado como spam por motor de IA/LLM (incluye nombre del motor, respuesta y conteo de tokens)
- **IsSpamRepeatComment** - Comentario marcado como spam por ser repetitivo (incluye qué motor lo detectó)
- **NotSpamIsOnlyImage** - Comentario no marcado como spam porque solo contiene imágenes
- **NotSpamIsOnlyReacts** - Comentario no marcado como spam porque solo contiene reacciones
- **NotSpamNoLinkOrMention** - Comentario no marcado como spam debido a ausencia de enlaces o menciones sospechosas
- **NotSpamPerfectTrustFactor** - Comentario no marcado como spam debido a un alto factor de confianza del usuario
- **NotSpamTooShort** - Comentario no marcado como spam porque es demasiado corto para analizar
- **NotSpamSkipped** - Se omitió la comprobación de spam
- **NotSpamFromEngine** - Comentario determinado como no spam por el motor de detección (incluye nombre del motor y factor de confianza)

### Bad Words/Profanity Events
- **BadWordsCheckFailed** - La comprobación del filtro de profanidad encontró un error
- **BadWordsFoundBadPhrase** - El filtro de profanidad detectó una frase inapropiada (incluye la frase)
- **BadWordsFoundBadWord** - El filtro de profanidad detectó una palabra inapropiada (incluye la palabra)
- **BadWordsNoDefinitionForLocale** - No hay definiciones de profanidad disponibles para el idioma del comentario (incluye la localidad)

### User Verification Events
- **CommentMustBeVerifiedToApproveNotInVerifiedSession** - El comentario requiere verificación pero el usuario no está en una sesión verificada
- **CommentMustBeVerifiedToApproveNotVerifiedYet** - El comentario requiere verificación pero el usuario aún no está verificado
- **InVerifiedSession** - El usuario que publica el comentario está en una sesión verificada
- **SentVerificationEmailNoSession** - Se envió un correo de verificación a un usuario no verificado
- **SentWelcomeEmail** - Se envió un correo de bienvenida al usuario nuevo

### Trust and Security Events
- **TrustFactorChanged** - El factor de confianza del usuario fue modificado (incluye valores antes y después)
- **SpamFilterDisabledBecauseAdmin** - El filtrado de spam fue omitido para un usuario administrador
- **TenantSpamFilterDisabled** - El filtrado de spam fue desactivado para todo el tenant
- **RepeatCommentCheckIgnored** - Se omitió la comprobación de comentarios repetidos (incluye la razón)
- **UserIsAdmin** - Usuario identificado como administrador
- **UserIsAdminParentTenant** - Usuario identificado como administrador del tenant principal
- **UserIsAdminViaSSO** - Usuario identificado como administrador vía SSO
- **UserIsMod** - Usuario identificado como moderador

### Comment Status Changes

Los eventos de cambio de estado incluyen valores antes y después, además del usuario que realizó el cambio:

- **ExpireStatusChanged** - El estado de expiración del comentario fue modificado
- **ReviewStatusChanged** - El estado de revisión del comentario fue cambiado
- **SpamStatusChanged** - El estado de spam del comentario fue actualizado
- **ApproveStatusChanged** - El estado de aprobación del comentario fue cambiado
- **TextChanged** - El contenido de texto del comentario fue editado (incluye texto antes y después)
- **VotesChanged** - Los recuentos de votos del comentario fueron actualizados (incluye desglose detallado de votos)
- **Flagged** - El comentario fue marcado por usuarios
- **UnFlagged** - Se eliminaron las marcas del comentario

### Moderation Actions
- **Pinned** - El comentario fue fijado por un moderador (incluye quién lo fijó)
- **UnPinned** - El comentario fue desafijado por un moderador (incluye quién lo desafijó)

### Notification Events
- **CreatedNotifications** - Se crearon notificaciones para el comentario (incluye el conteo de notificaciones)
- **NotificationCreateFailure** - Error al crear notificaciones
- **BadgeAwarded** - Se otorgó una insignia de usuario por el comentario (incluye el nombre de la insignia)

### Publishing Events
- **PublishedLive** - El comentario fue publicado a suscriptores en vivo (incluye el conteo de suscriptores)

### Integration Events
- **WebhookSynced** - El comentario fue sincronizado vía webhook

### Spam Rule Events
- **SpamRuleMatch** - El comentario coincidió con una regla de spam personalizada (incluye detalles de la regla)

### Localization Events
- **LocaleDetectedFromText** - La localidad de idioma fue detectada automáticamente a partir del texto del comentario (incluye el idioma detectado y la localidad)

## Use Cases for Comment Logs

Los registros de comentarios se generan y almacenan automáticamente con cada comentario. Proporcionan información valiosa para:

- **Comprender las decisiones de moderación** - Ver exactamente por qué un comentario fue aprobado, retenido para revisión o marcado como spam
- **Depurar problemas de aprobación/spam** - Rastrear la lógica de decisión cuando los comentarios no funcionan como se espera
- **Rastrear patrones de comportamiento de usuarios** - Monitorear cambios en el factor de confianza y el estado de verificación
- **Auditar acciones de moderadores** - Revisar qué acciones han tomado los moderadores sobre comentarios específicos
- **Investigar la efectividad del filtro de spam** - Ver qué motores de detección están capturando spam y cuáles no
- **Solucionar problemas de integraciones** - Verificar sincronizaciones de webhook y entrega de notificaciones

Estos registros ayudan a mantener la transparencia en el proceso de moderación y asisten en el ajuste fino del comportamiento de tu sistema de comentarios.