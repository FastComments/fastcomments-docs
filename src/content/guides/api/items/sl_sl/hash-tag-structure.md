Objekt `HashTag` predstavlja oznako, ki jo lahko pusti uporabnik. HashTags lahko uporabite za povezavo do zunanje vsebine ali za povezovanje povezanih komentarjev.

Struktura objekta `HashTag` je naslednja:

[inline-code-attrs-start title = 'Struktura HashTag'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

Opombe:

- V nekaterih API končnih točkah boste videli, da je hashtag uporabljen v URL-ju. Ne pozabite na URI-kodirane vrednosti. Na primer, `#` je treba namesto tega predstaviti kot `%23`.
- Nekatera od teh polj so označena kot `READONLY` - API jih vrne, vendar jih ni mogoče nastaviti.