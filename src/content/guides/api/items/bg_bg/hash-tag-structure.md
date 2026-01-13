Обектът `HashTag` представлява таг, който може да бъде оставен от потребител. HashTags могат да се използват за свързване към външно съдържание или за
свързване на свързани коментари заедно.

Структурата на обекта `HashTag` е следната:

[inline-code-attrs-start title = 'Структура на HashTag'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

Забележки:

- В някои API крайни точки ще видите, че хаштагът се използва в URL. Не забравяйте да URI-кодирате стойностите. Например `#` трябва вместо това да се представи като `%23`.
- Някои от тези полета са маркирани като `READONLY` - те се връщат от API, но не могат да бъдат зададени.
