Un objeto `FeedPost` representa una publicación en un feed de FastComments. Un feed es una secuencia de publicaciones con sus propios hilos de comentarios, renderizado por el widget Feed. Cada publicación tiene un autor, contenido enriquecido opcional, medios y enlaces, y puede ser etiquetada para que un feed pueda filtrarse.

La estructura del objeto `FeedPost` es la siguiente:

[inline-code-attrs-start title = 'Estructura de FeedPost'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPost {
    /** SOLO LECTURA **/
    _id: string
    /** SOLO LECTURA **/
    tenantId: string
    title?: string
    /** El id del usuario de FastComments o SSO que creó la publicación. **/
    fromUserId?: string
    /** Rellenado a partir del usuario cuando no está establecido. **/
    fromUserDisplayName?: string | null
    /** SOLO LECTURA. Rellenado a partir del usuario. **/
    fromUserAvatar?: string | null
    /** Usado para filtrar un feed. **/
    tags?: string[]
    /** Peso de ordenación dentro de un feed. Los valores más altos se ordenan primero. **/
    weight?: number
    /** Pares clave/valor de forma libre para su propio uso. **/
    meta?: Record<string, string>
    /** HTML sanitizado. **/
    contentHTML?: string
    media?: FeedPostMediaItem[]
    links?: FeedPostLink[]
    /** SOLO LECTURA **/
    createdAt: string
    /** SOLO LECTURA. Tipo de reacción a contar. **/
    reacts?: Record<string, number>
    /** SOLO LECTURA **/
    commentCount?: number | null
}

interface FeedPostMediaItem {
    title?: string
    /** A dónde enlaza el elemento multimedia al hacer clic. **/
    linkUrl?: string
    /** Una entrada por cada versión. El widget elige la mejor opción. **/
    sizes: FeedPostMediaItemAsset[]
}

interface FeedPostMediaItemAsset {
    w: number
    h: number
    src: string
}

interface FeedPostLink {
    /** El texto del enlace, como "Regístrate ahora". **/
    text?: string
    /** Un encabezado mostrado con el enlace. **/
    title?: string
    /** Una descripción mostrada con el enlace. **/
    description?: string
    url?: string
}
[inline-code-end]

Notas:

- Algunos de estos campos están marcados como `READONLY` - se devuelven por la API pero no pueden ser establecidos.
- Los comentarios en una publicación son comentarios regulares cuyo `urlId` es `post:` seguido del `_id` de la publicación. Use ese valor con la API de Comentarios para leer o crear comentarios en una publicación.