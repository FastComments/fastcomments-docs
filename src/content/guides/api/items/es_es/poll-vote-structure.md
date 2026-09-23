A `PollVote` es la respuesta de una persona a una encuesta. Los recuentos mostrados en la propia encuesta se mantienen sincronizados con estos, por lo que solo necesitas estos cuando quieres saber *quién* votó por qué, en lugar de los totales.

Un votante tiene como máximo un voto por encuesta. Votar de nuevo mueve su voto existente a la nueva opción en lugar de agregar uno segundo, y `updatedAt` registra cuándo ocurrió eso.

`voterId` es el `userId` cuando el votante estaba conectado, y el `anonUserId` en caso contrario.

[inline-code-attrs-start title = 'Estructura PollVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVote {
    id: string
    tenantId: string
    commentId: string
    urlId: string
    /** El userId cuando el votante estaba conectado, de lo contrario el anonUserId. **/
    voterId: string
    optionId: string
    createdAt: string
    /** Cuando el votante movió su voto por última vez a una opción diferente. **/
    updatedAt?: string
}
[inline-code-end]

### Privacidad

La configuración `privacy` de la encuesta se aplica a esta API de la misma manera que se aplica en el widget de comentarios:

- **Anónimo** (por defecto): nadie puede ver cómo votó nadie, por lo que los votos no pueden leerse.
  `GET /api/v1/poll-votes` y `GET /api/v1/poll-votes/:id` responden con `poll-anonymous`. Los recuentos de la encuesta
  siguen estando disponibles desde `GET /api/v1/polls/:commentId`.
- **Administradores y moderadores**: tu clave API pertenece al administrador de tu sitio, por lo que puede leer los votos.
- **Todos**: los votos pueden leerse.

La privacidad de la encuesta puede estrecharse pero no ampliarse una vez que tiene votos.

---