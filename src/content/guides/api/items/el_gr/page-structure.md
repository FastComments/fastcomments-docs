Ένα αντικείμενο `Page` αντιπροσωπεύει τη σελίδα στην οποία μπορούν να ανήκουν πολλά σχόλια. Αυτή η σχέση ορίζεται από
το `urlId`.

Μια `Page` αποθηκεύει πληροφορίες όπως ο τίτλος σελίδας, ο αριθμός σχολίων και το `urlId`.

Η δομή για το αντικείμενο Page είναι η ακόλουθη:

[inline-code-attrs-start title = 'Δομή Σελίδας'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Page {
    id: string
    urlId: string
    url: string
    title?: string
    createdAt: string
    commentCount: number
    rootCommentCount: number
    /** Setting this to null means all SSO users can see the page. An empty list means it is closed to all users. **/
    accessibleByGroupIds?: string[] | null
    /** Is this page closed for new comments? **/
    isClosed?: boolean
}
[inline-code-end]
