Об'єкт `HashTag` представляє тег, який може залишити користувач. Хештеги можуть використовуватися для зв'язування з зовнішнім вмістом або для об'єднання пов'язаних коментарів.

Структура об'єкта `HashTag` виглядає так:

[inline-code-attrs-start title = 'Структура HashTag'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface HashTag {
    /** Повинно починатися з символу "#" або іншого бажаного символу. **/
    tag: string
    /** Необов'язковий URL, на який може вказувати хештег. Замість фільтрації коментарів за хештегом, інтерфейс перенаправить на цей URL при натисканні. **/
    url?: string
    /** ТІЛЬКИ ДЛЯ ЧИТАННЯ **/
    createdAt: string
}
[inline-code-end]

Notes:

- In some API endpoints you will see that the hashtag is used in the URL. Remember to URI-Encoded values. For example, `#` should instead be represented as `%23`.
- Some of these fields are marked `READONLY` - these are returned by the API but cannot be set.