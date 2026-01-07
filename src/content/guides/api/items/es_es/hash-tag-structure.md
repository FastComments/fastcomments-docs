Un objeto `HashTag` representa una etiqueta que puede ser dejada por un usuario. Los HashTags pueden usarse para vincular a una pieza externa de contenido o para
vincular comentarios relacionados juntos.

La estructura del objeto `HashTag` es la siguiente:

[inline-code-attrs-start title = 'Estructura de HashTag'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

Notas:

- En algunos endpoints de API verá que el hashtag se usa en la URL. Recuerde usar valores codificados en URI. Por ejemplo, `#` debe representarse como `%23`.
- Algunos de estos campos están marcados como `READONLY` - estos son devueltos por la API pero no pueden ser establecidos.

