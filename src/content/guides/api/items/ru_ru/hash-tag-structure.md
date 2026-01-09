---
A `HashTag` object represents a tag that can be left by a user. HashTags can be used to link to an external piece of content or to
tie related comments together.

The structure for the `HashTag` object is as follows:

[inline-code-attrs-start title = 'Структура HashTag'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface HashTag {
    /** Должен начинаться с символа "#" или другого желаемого символа. **/
    tag: string
    /** Необязательный URL, на который может указывать хештег. Вместо фильтрации комментариев по хештегу интерфейс при клике перенаправит на этот URL. **/
    url?: string
    /** Только для чтения **/
    createdAt: string
}
[inline-code-end]

Notes:

- In some API endpoints you will see that the hashtag is used in the URL. Remember to URI-Encoded values. For example, `#` should instead be represented as `%23`.
- Some of these fields are marked `READONLY` - these are returned by the API but cannot be set.
 

---