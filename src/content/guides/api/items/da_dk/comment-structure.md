Et `Comment`-objekt repræsenterer en enkelt kommentar.

Strukturen for Comment-objektet er som følger:

[inline-code-attrs-start title = 'Comment Struktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Comment {
    id: string
    oderId: string
    urlId: string
    url: string
    pageTitle: string
    userId: string
    commenterEmail: string
    commenterName: string
    comment: string
    commentHTML: string
    /** This is the raw markdown from the user. **/
    commentText: string
    parentId: string | null
    date: string
    votes: number
    votesUp: number
    votesDown: number
    verified: boolean
    verifiedDate: string
    avatarSrc: string
    hasImages: boolean
    approved: boolean
    isSpam: boolean
    aiDeterminedSpam: boolean
    reviewed: boolean
    imported: boolean
    /** This is used to override the default locale in emails (for example when replying to this user). This is NOT the locale the comment was left in. **/
    notificationLocaleOverride: string
    /** Used with SSO to store any id you want. Max length 200 characters. Can be used with GET Comment APIs to filter by externalId. **/
    externalId: string | null
    /** This is an obfuscated version of the externalId for client-side rendering. **/
    externalIdHash: string | null
    /** The comment the user replied to. Only present in some situations. **/
    replyingTo?: Comment
}
[inline-code-end]

### Kommentarstreng Felter

Bemærk: Når du henter kommentarer via API'et, og du vil gengive rå tekst-markdown'en, brug `commentText`.

`comment` er en "visningsoptimeret" og ryddet version af kommentaren. Hvis for eksempel den originale kommentar var:

`### En overskrift` og ingen HTML var aktiveret, vil `comment` være `En overskrift`, uden formatering eller kommentar-prefix. Feltet `commentHTML` vil indeholde HTML-koden til at gengive kommentaren. Disse felter kan begge være `null`.

Kun `commentText` vil aldrig være `null`.

### Brugerdefinerede Metadata

Du kan gemme et nøgle-værdi-par med kommentaren, som du kan bruge til at forespørge. Nøgler må ikke indeholde "." eller "$" eller være længere end 100 tegn. Værdier må ikke være længere end 2k tegn.

#### Eksempel

[inline-code-attrs-start title = 'Metadata Eksempel'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
    "meta": {
        "some-key": "some-value"
    }
}
[inline-code-end]
