Объект `HashTag` представляет собой метку, которую может оставить пользователь. HashTags могут использоваться для ссылки на внешний фрагмент контента или для
объединения связанных комментариев.

The structure for the `HashTag` object is as follows:

[inline-code-attrs-start title = 'Структура HashTag'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface HashTag {
    /** Должен начинаться с "#" или другого желаемого символа. **/
    tag: string
    /** Необязательный URL, на который может указывать хэштег. Вместо фильтрации комментариев по хэштегу интерфейс перенаправит на этот URL при клике. **/
    url?: string
    /** ТОЛЬКО ДЛЯ ЧТЕНИЯ **/
    createdAt: string
}
[inline-code-end]

Notes:

- In some API endpoints you will see that the hashtag is used in the URL. Remember to URI-Encoded values. For example, `#` should instead be represented as `%23`.
- Some of these fields are marked `READONLY` - these are returned by the API but cannot be set.