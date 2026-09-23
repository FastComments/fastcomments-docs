Una `Poll` está adjunta a un comentario, en lugar de ser un objeto independiente. Se crea con el comentario (ver `POST /api/v1/comments`), o se agrega a un comentario existente más tarde con `PUT /api/v1/polls/:commentId`.

Los recuentos de votos se guardan en la propia encuesta, por lo que leer una encuesta le brinda los resultados sin necesidad de sumarlos. Los votos individuales detrás de esos recuentos son objetos `PollVote`.

Cada opción tiene un `id` que se genera cuando se crea la encuesta. Ese id es lo que se usa para emitir un voto, para volver a etiquetar una opción y para conservar una opción (y sus votos) cuando se `PUT` la encuesta con opciones añadidas o eliminadas. Es la única forma segura de referirse a una opción, nunca su posición en la lista.

[inline-code-attrs-start title = 'Estructura de la encuesta'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentPollOption {
    id: string
    label: string
    votes: number
}

interface CommentPoll {
    question: string
    options: CommentPollOption[]
    totalVotes: number
    /** When set and in the past, the poll is closed and no longer accepts votes. **/
    closesAt?: string | null
    /** 0 anonymous (the default), 1 admins and moderators, 2 everyone. Absent means anonymous. **/
    privacy?: 0 | 1 | 2 | null
    /** When true, the counts are hidden from anyone who has not voted yet. Absent means false. **/
    requireVoteToSeeResults?: boolean | null
}
[inline-code-end]

### Límites

- Se requiere una pregunta, y tiene como máximo 200 caracteres.
- Una encuesta tiene entre 2 y 10 opciones.
- Se requiere una etiqueta de opción, tiene como máximo 100 caracteres y debe ser única dentro de la encuesta (ignorando mayúsculas/minúsculas).
- `closesAt` debe estar en el futuro cuando se crea la encuesta. Para cerrar una encuesta inmediatamente, `PATCH`la con una fecha en el pasado.

### Configuración del sitio

Las encuestas obedecen la configuración de su sitio, que puede cambiar bajo Personalizar Widget:

- Las encuestas deben estar habilitadas antes de que se pueda crear una encuesta, o la API responde con `polls-disabled`.
- La votación puede limitarse a usuarios registrados, en cuyo caso un voto enviado solo con un `anonUserId` es rechazado con `poll-login-required`.