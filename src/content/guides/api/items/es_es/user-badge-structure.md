`UserBadge` es un objeto que representa una insignia asignada a un usuario en el sistema FastComments.

Las insignias pueden asignarse a los usuarios automáticamente según su actividad (como el número de comentarios, el tiempo de respuesta, el estado de veterano) o manualmente por los administradores del sitio.

La estructura del objeto `UserBadge` es la siguiente:

[inline-code-attrs-start title = 'Estructura de UserBadge'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export interface UserBadge {
    /** Identificador único para esta asignación de insignia de usuario */
    id: string
    /** ID del usuario al que se asigna esta insignia */
    userId: string
    /** ID de la definición de insignia del catálogo de insignias del tenant */
    badgeId: string
    /** ID del tenant que creó/asignó esta insignia */
    fromTenantId: string
    /** Cuando se creó esta insignia (milisegundos desde el epoch) */
    createdAt?: number
    /** Cuando el usuario recibió esta insignia (milisegundos desde el epoch) */
    receivedAt?: number
    /** 
     * El tipo de insignia: 
     * 0=CommentCount, 1=CommentUpVotes, 2=CommentReplies, 3=CommentsPinned, 
     * 4=Veteran, 5=NightOwl, 6=FastReplyTime, 7=ModeratorCommentsDeleted,
     * 8=ModeratorCommentsApproved, 9=ModeratorCommentsUnapproved, 10=ModeratorCommentsReviewed,
     * 11=ModeratorCommentsMarkedSpam, 12=ModeratorCommentsMarkedNotSpam, 13=RepliedToSpecificPage,
     * 14=Manual
     */
    type: number
    /** Para insignias basadas en umbral, el valor del umbral */
    threshold?: number
    /** El nombre/etiqueta de la insignia */
    name?: string
    /** Descripción detallada de la insignia */
    description?: string
    /** El texto mostrado en la insignia */
    displayLabel?: string
    /** URL de una imagen mostrada en la insignia */
    displaySrc?: string
    /** Color de fondo de la insignia (código hexadecimal) */
    backgroundColor?: string
    /** Color de borde de la insignia (código hexadecimal) */
    borderColor?: string
    /** Color del texto de la insignia (código hexadecimal) */
    textColor?: string
    /** Clase CSS adicional para el estilo */
    cssClass?: string
    /** Para insignias de veterano, el umbral de tiempo en milisegundos */
    veteranUserThresholdMillis?: number
    /** Si esta insignia se muestra en los comentarios del usuario */
    displayedOnComments: boolean
    /** El orden de visualización de la insignia */
    order?: number
    /** Si se establece, esta insignia solo se muestra en la página con el urlId correspondiente. Null para insignias globales. */
    urlId?: string | null
}
[inline-code-end]
---