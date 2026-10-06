FastComments rastrea automáticamente eventos detallados para cada comentario para proporcionar transparencia en las decisiones de moderación y acciones del sistema. Estos registros le ayudan a comprender por qué un comentario fue aprobado, marcado como spam o tuvo su estado cambiado.

## Accediendo a los Registros de Comentarios

Para ver los registros de un comentario específico:

1. Navegue a la página **Moderar Comentarios** en su panel de FastComments  
2. Encuentre el comentario que desea inspeccionar  
3. Haga clic en el botón **Ver Registros** (icono de reloj) en la barra de acciones del comentario  
4. Aparecerá un cuadro de diálogo que muestra el historial completo de eventos para ese comentario  

Cada entrada de registro muestra:
- **When** – La marca de tiempo del evento  
- **Who** – El usuario o sistema que desencadenó el evento (cuando corresponda)  
- **What** – El tipo de acción o evento  
- **Details** – Contexto adicional como valores antes/después, nombres de motor o datos relacionados  

## Eventos de Registro de Comentarios

Cada comentario mantiene un registro de eventos que ocurren durante su ciclo de vida. A continuación se enumeran los tipos de eventos que se rastrean:

### Eventos de Anonimización
- **Anonymized** – El contenido del comentario se borró y el usuario se marcó como eliminado  
- **RestoredFromAnonymized** – El comentario se restauró desde el estado anonimizado  

### Eventos de Aprobación
- **ApprovedDueToPastComment** – Comentario aprobado porque el usuario tiene comentarios aprobados previamente (incluye referencia al comentario pasado)  
- **ApprovedIsAdmin** – Comentario aprobado porque el usuario es administrador  
- **NotApprovedRequiresApproval** – Comentario requiere aprobación manual  
- **NotApprovedLowTrustFactor** – Comentario no aprobado debido a un bajo factor de confianza del usuario (incluye el valor del factor de confianza)  

### Eventos de Aprobación de Comentario de Perfil

Estos eventos se aplican específicamente a los comentarios en perfiles de usuario:

- **ApprovedProfileAutoApproveAll** – Comentario de perfil autoaprobado porque el propietario del perfil ha habilitado la autoaprobación para todos los comentarios  
- **ApprovedProfileTrusted** – Comentario de perfil aprobado porque el comentarista es de confianza (incluye referencia al comentario que estableció la confianza)  
- **NotApprovedProfileManualApproveAll** – Comentario de perfil requiere aprobación manual porque el propietario del perfil ha habilitado la aprobación manual  
- **NotApprovedProfileNotTrusted** – Comentario de perfil no aprobado porque el comentarista no es de confianza  
- **NotApprovedProfileNewUser** – Comentario de perfil no aprobado porque el comentarista es un usuario nuevo  

### Eventos de Detección de Spam
- **IsSpam** – Comentario marcado como spam por el motor de detección (incluye qué motor tomó la decisión)  
- **IsSpamDueToBadWords** – Comentario marcado como spam debido al filtro de palabras ofensivas  
- **IsSpamFromLLM** – Comentario marcado como spam por motor de IA/LLM (incluye nombre del motor, respuesta y recuento de tokens)  
- **IsSpamRepeatComment** – Comentario marcado como spam por ser repetitivo (incluye qué motor lo detectó)  
- **NotSpamIsOnlyImage** – Comentario no marcado como spam porque solo contiene imágenes  
- **NotSpamIsOnlyReacts** – Comentario no marcado como spam porque solo contiene reacciones  
- **NotSpamNoLinkOrMention** – Comentario no marcado como spam por no contener enlaces o menciones sospechosas  
- **NotSpamPerfectTrustFactor** – Comentario no marcado como spam debido a un alto factor de confianza del usuario  
- **NotSpamTooShort** – Comentario no marcado como spam porque es demasiado corto para analizar  
- **NotSpamSkipped** – La verificación de spam se omitió  
- **NotSpamFromEngine** – Comentario determinado como no spam por el motor de detección (incluye nombre del motor y factor de confianza)  

### Eventos de Palabras Ofensivas/Profanidad
- **BadWordsCheckFailed** – La verificación del filtro de profanidad encontró un error  
- **BadWordsFoundBadPhrase** – El filtro de profanidad detectó una frase inapropiada (incluye la frase)  
- **BadWordsFoundBadWord** – El filtro de profanidad detectó una palabra inapropiada (incluye la palabra)  
- **BadWordsNoDefinitionForLocale** – No hay definiciones de profanidad disponibles para el idioma del comentario (incluye la configuración regional)  

### Eventos de Verificación de Usuario
- **CommentMustBeVerifiedToApproveNotInVerifiedSession** – Comentario requiere verificación pero el usuario no está en una sesión verificada  
- **CommentMustBeVerifiedToApproveNotVerifiedYet** – Comentario requiere verificación pero el usuario aún no está verificado  
- **InVerifiedSession** – El usuario que publica el comentario está en una sesión verificada  
- **SentVerificationEmailNoSession** – Correo de verificación enviado a usuario no verificado  
- **SentWelcomeEmail** – Correo de bienvenida enviado a nuevo usuario  

### Eventos de Confianza y Seguridad
- **TrustFactorChanged** – El factor de confianza del usuario se modificó (incluye valores antes y después)  
- **SpamFilterDisabledBecauseAdmin** – Filtrado de spam omitido para usuario administrador  
- **TenantSpamFilterDisabled** – Filtrado de spam deshabilitado para todo el inquilino  
- **RepeatCommentCheckIgnored** – Verificación de comentario repetido omitida (incluye la razón)  
- **UserIsAdmin** – Usuario identificado como administrador  
- **UserIsAdminParentTenant** – Usuario identificado como administrador del inquilino principal  
- **UserIsAdminViaSSO** – Usuario identificado como administrador vía SSO  
- **UserIsMod** – Usuario identificado como moderador  

### Cambios de Estado del Comentario

Los eventos de cambio de estado incluyen valores antes y después, además del usuario que realizó el cambio:

- **ExpireStatusChanged** – El estado de expiración del comentario se modificó  
- **ReviewStatusChanged** – El estado de revisión del comentario cambió  
- **SpamStatusChanged** – El estado de spam del comentario se actualizó  
- **ApproveStatusChanged** – El estado de aprobación del comentario cambió  
- **TextChanged** – El contenido de texto del comentario se editó (incluye texto antes y después)  
- **VotesChanged** – Los recuentos de votos del comentario se actualizaron (incluye desglose detallado de votos)  
- **Flagged** – Comentario fue marcado por usuarios  
- **UnFlagged** – Se eliminaron las marcas del comentario  

### Acciones de Moderación
- **Pinned** – Comentario fijado por moderador (incluye quién lo fijó)  
- **UnPinned** – Comentario desafijado por moderador (incluye quién lo desafijó)  

### Eventos de Notificación
- **CreatedNotifications** – Se crearon notificaciones para el comentario (incluye recuento de notificaciones)  
- **NotificationCreateFailure** – Error al crear notificaciones  
- **BadgeAwarded** – Se otorgó una insignia al usuario por el comentario (incluye nombre de la insignia)  

### Eventos de Mención y Notificación de Respuesta

Estos eventos nombran a la persona que recibiría el correo electrónico o la notificación. Cuando no se envió nada, la columna Detalles indica el motivo.

- **MentionEmailSent** – Un usuario mencionado en el comentario recibió un correo electrónico  
- **MentionEmailSkipped** – Un usuario mencionado no recibió correo electrónico (incluye la razón)  
- **MentionHeldForApproval** – El correo de mención está en espera hasta que el comentario sea aprobado  
- **MentionNotificationCreated** – Un usuario mencionado recibió una notificación dentro de la aplicación  
- **MentionNotificationSkipped** – Un usuario mencionado no recibió una notificación dentro de la aplicación (incluye la razón)  
- **ReplyEmailSent** – El autor del comentario al que se respondió recibió un correo electrónico sobre esta respuesta  
- **ReplyEmailSkipped** – El autor del comentario al que se respondió no recibió correo electrónico (incluye la razón)  
- **ReplyNotificationSkipped** – El autor del comentario al que se respondió no recibió una notificación dentro de la aplicación (incluye la razón)  

Razones mostradas cuando un correo electrónico o notificación no se envió:

- El usuario ya no existe, o no tiene dirección de correo electrónico  
- El usuario desactivó las notificaciones por correo, o desactivó las notificaciones para ese hilo  
- Uno de los dos usuarios ha bloqueado al otro  
- Los usuarios no están en ninguno de los mismos grupos SSO  
- La dirección de correo del usuario está en la lista de supresión después de un rebote o queja de spam (ver [Email Suppression Management](/guide-notifications.html#email-suppression-management))  
- La dirección de correo del usuario es example.com, que no puede recibir correo  
- El comentario se marcó como spam, se eliminó o no se aprobó dentro de los 7 días  
- El comentario al que se respondió se dejó de forma anónima  
- El usuario respondió a su propio comentario  
- El usuario fue mencionado en la respuesta, por lo que recibió el correo de mención en lugar del correo de respuesta  
- El usuario ya tenía una notificación de respuesta para el comentario  
- El envío falló 5 veces  

Si la entrega falla o alcanza un límite de envío, el correo se coloca en cola para reintentar y la entrada del registro lo indica.

### Eventos de Publicación
- **PublishedLive** – Comentario publicado a suscriptores en vivo (incluye recuento de suscriptores)  

### Eventos de Integración
- **WebhookSynced** – Comentario sincronizado vía webhook  

### Eventos de Regla de Spam
- **SpamRuleMatch** – Comentario coincidió con una regla de spam personalizada (incluye detalles de la regla)  

### Eventos de Localización
- **LocaleDetectedFromText** – La configuración regional del idioma se detectó automáticamente a partir del texto del comentario (incluye idioma y configuración regional detectados)  

## Casos de Uso para los Registros de Comentarios

Los registros de comentarios se generan automáticamente y se almacenan con cada comentario. Proporcionan información valiosa para:

- **Entender decisiones de moderación** – Ver exactamente por qué un comentario fue aprobado, retenido para revisión o marcado como spam  
- **Depurar problemas de aprobación/spam** – Rastrear la lógica de decisiones cuando los comentarios no se comportan como se espera  
- **Rastrear patrones de comportamiento de usuarios** – Monitorizar cambios en el factor de confianza y estado de verificación  
- **Auditar acciones de moderadores** – Revisar qué acciones han tomado los moderadores en comentarios específicos  
- **Investigar la efectividad del filtro de spam** – Ver qué motores de detección están atrapando spam y cuáles no  
- **Solucionar problemas de integraciones** – Verificar sincronizaciones de webhook y entrega de notificaciones  

Estos registros ayudan a mantener la transparencia en el proceso de moderación y a afinar el comportamiento de su sistema de comentarios.