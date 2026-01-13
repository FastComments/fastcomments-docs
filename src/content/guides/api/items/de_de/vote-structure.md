Ein `Vote`-Objekt repräsentiert eine Abstimmung, die von einem Benutzer hinterlassen wurde.

Die Beziehung zwischen Kommentaren und Abstimmung wird über `commentId` definiert.

Die Struktur des Vote-Objekts ist wie folgt:

[inline-code-attrs-start title = 'Vote Struktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
