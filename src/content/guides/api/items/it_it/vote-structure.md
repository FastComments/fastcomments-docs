---
Un oggetto `Vote` rappresenta un voto lasciato da un utente.

La relazione tra commenti e il voto è definita tramite `commentId`.

La struttura dell'oggetto Vote è la seguente:

[inline-code-attrs-start title = 'Struttura oggetto Vote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Vote {
    id: string
    urlId: string
    commentId: string
    userId: string
    direction: 1 | -1
    createdAt: string
}
[inline-code-end]

---