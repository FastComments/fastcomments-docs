Et `FeedPost`-objekt repræsenterer et indlæg i et FastComments-feed. Et feed er en strøm af indlæg med deres egne kommentartråde, gengivet af Feed‑widgeten. Hvert indlæg har en forfatter, valgfrit rigt indhold, medier og links, og kan blive mærket så et feed kan filtreres.

Strukturen for `FeedPost`-objektet er som følger:

[inline-code-attrs-start title = 'FeedPost Struktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPost {
    /** READONLY **/
    _id: string
    /** READONLY **/
    tenantId: string
    title?: string
    /** ID'et for FastComments- eller SSO-brugeren, der har forfattet indlægget. **/
    fromUserId?: string
    /** Udfyldt fra brugeren, hvis ikke angivet. **/
    fromUserDisplayName?: string | null
    /** READONLY. Udfyldt fra brugeren. **/
    fromUserAvatar?: string | null
    /** Brugt til at filtrere et feed. **/
    tags?: string[]
    /** Sorteringsvægt inden for et feed. Højere værdier sorteres først. **/
    weight?: number
    /** Friform nøgle/værdi-par til eget brug. **/
    meta?: Record<string, string>
    /** Sanitiseret HTML. **/
    contentHTML?: string
    media?: FeedPostMediaItem[]
    links?: FeedPostLink[]
    /** READONLY **/
    createdAt: string
    /** READONLY. Reaktionstype at tælle. **/
    reacts?: Record<string, number>
    /** READONLY **/
    commentCount?: number | null
}

interface FeedPostMediaItem {
    title?: string
    /** Hvor medieelementet linker til, når der klikkes. **/
    linkUrl?: string
    /** Én indgang per gengivelse. Widget'en vælger den bedste pasning. **/
    sizes: FeedPostMediaItemAsset[]
}

interface FeedPostMediaItemAsset {
    w: number
    h: number
    src: string
}

interface FeedPostLink {
    /** Linkteksten, f.eks. "Tilmeld dig nu". **/
    text?: string
    /** En overskrift vist sammen med linket. **/
    title?: string
    /** En beskrivelse vist sammen med linket. **/
    description?: string
    url?: string
}
[inline-code-end]

Noter:

- Nogle af disse felter er markeret `READONLY` – de returneres af API'et, men kan ikke sættes.
- Kommentarerne på et indlæg er almindelige kommentarer, hvis `urlId` er `post:` efterfulgt af indlæggets `_id`. Brug den værdi med Kommentar‑API'et for at læse eller oprette kommentarer på et indlæg.