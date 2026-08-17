A `HashTag` object представляет тег, который может быть оставлен пользователем. HashTag'и могут использоваться для ссылки на внешний контент или для связывания связанных комментариев.

Структура объекта `HashTag` выглядит следующим образом:

[inline-code-attrs-start title = 'Структура HashTag'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

Примечания:

- В некоторых конечных точках API вы увидите, что хэштег используется в URL. Не забудьте кодировать значения в URI. Например, `#` следует представлять как `%23`.
- Некоторые из этих полей помечены как `READONLY` — они возвращаются API, но не могут быть заданы.