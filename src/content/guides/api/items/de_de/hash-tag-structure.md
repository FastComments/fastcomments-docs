Ein `HashTag`-Objekt repräsentiert ein Tag, das von einem Benutzer hinterlassen werden kann. HashTags können verwendet werden, um auf einen externen Inhalt zu verlinken oder um
zusammengehörige Kommentare zu verbinden.

Die Struktur des `HashTag`-Objekts ist wie folgt:

[inline-code-attrs-start title = 'HashTag Struktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface HashTag {
    /** Should start with the "#" or desired character. **/
    tag: string
    /** An optional URL that the hashtag can point to. Instead of filtering comments by hashtag, the UI will redirect to this upon click. **/
    url?: string
    /** READONLY **/
    createdAt: string
}
[inline-code-end]

Hinweise:

- In einigen API-Endpunkten werden Sie sehen, dass der Hashtag in der URL verwendet wird. Denken Sie daran, Werte URL-zu-kodieren. Zum Beispiel sollte `#` stattdessen als `%23` dargestellt werden.
- Einige dieser Felder sind als `READONLY` markiert - diese werden von der API zurückgegeben, können aber nicht gesetzt werden.
