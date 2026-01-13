Objekt `HashTag` predstavlja oznako, ki jo lahko pusti uporabnik. HashTags se lahko uporabijo za povezavo na zunanjo vsebino ali za
povezovanje sorodnih komentarjev.

The structure for the `HashTag` object is as follows:

[inline-code-attrs-start title = 'Struktura HashTag'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface HashTag {
    /** Naj se začne z "#" ali želenim znakom. **/
    tag: string
    /** Izbiren URL, na katerega lahko kaže hashtag. Namesto filtriranja komentarjev po hashtagu bo uporabniški vmesnik ob kliku preusmeril na ta URL. **/
    url?: string
    /** LE ZA BRANJE **/
    createdAt: string
}
[inline-code-end]

Notes:

- In some API endpoints you will see that the hashtag is used in the URL. Remember to URI-Encoded values. For example, `#` should instead be represented as `%23`.
- Some of these fields are marked `READONLY` - these are returned by the API but cannot be set.
 

---