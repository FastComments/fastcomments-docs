Bir `Page` nesnesi, birçok yorumun ait olabileceği sayfayı temsil eder. Bu ilişki şu şekilde tanımlanır
`urlId`.

Bir `Page`, sayfa başlığı, yorum sayısı ve `urlId` gibi bilgileri saklar.

Page nesnesinin yapısı aşağıdaki gibidir:

[inline-code-attrs-start title = 'Sayfa Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Page {
    id: string
    urlId: string
    url: string
    title?: string
    createdAt: string
    commentCount: number
    rootCommentCount: number
    /** Bunu null olarak ayarlamak, tüm SSO kullanıcılarının sayfayı görebileceği anlamına gelir. Boş bir liste ise tüm kullanıcılara kapalı olduğu anlamına gelir. **/
    accessibleByGroupIds?: string[] | null
    /** Bu sayfa yeni yorumlara kapalı mı? **/
    isClosed?: boolean
}
[inline-code-end]

---