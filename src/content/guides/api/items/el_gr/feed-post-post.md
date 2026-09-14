[api-resource-header-start name = 'FeedPost'; route = 'POST /api/v1/feed-posts'; creditsCost = 1; api-resource-header-end]

Αυτή η διαδρομή δημιουργεί ένα μοναδικό `FeedPost`. Κάθε δημοσίευση έχει έναν συγγραφέα, επομένως το `fromUserId` είναι υποχρεωτικό και πρέπει να είναι το αναγνωριστικό ενός υπάρχοντος χρήστη FastComments ή SSO στον λογαριασμό.

[inline-code-attrs-start title = 'Παράδειγμα cURL Δημιουργίας FeedPost'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/feed-posts?tenantId=demo&isLive=true&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "fromUserId": "some-user-id",
    "title": "Release 2.0 is out",
    "contentHTML": "<p>Read the notes and tell us what you think.</p>",
    "tags": ["releases"],
    "links": [
        {
            "url": "https://example.com/releases/2.0",
            "title": "Release notes",
            "description": "Everything that changed in 2.0."
        }
    ]
}'
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Αιτήματος Δημιουργίας FeedPost'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPostPostQueryParams {
    tenantId: string
    API_KEY: string
    /** Σπρώχνει τη δημοσίευση σε ροές που είναι ανοιχτές σε έναν περιηγητή αυτή τη στιγμή. Προεπιλογή είναι false. **/
    isLive?: boolean
    /** Εκτελεί τη δημοσίευση μέσω του μηχανισμού spam πριν την αποθήκευση. Προεπιλογή είναι false. **/
    doSpamCheck?: boolean
    /** Παραλείπει τον έλεγχο επαναλαμβανόμενου περιεχομένου που εκτελείται ως μέρος του doSpamCheck. Προεπιλογή είναι false. **/
    skipDupCheck?: boolean
    /** Μέχρι 256 χαρακτήρες. Επαναλαμβάνεται στους ζωντανούς ακροατές ώστε ένας πελάτης να μπορεί να αγνοήσει τη δική του μετάδοση. **/
    broadcastId?: string
}

interface FeedPostPostBody {
    /** Απαιτείται. Ένα αναγνωριστικό χρήστη FastComments ή SSO. **/
    fromUserId: string
    title?: string
    /** HTML. Καθαρίζεται κατά την αποθήκευση. **/
    contentHTML?: string
    /** Αντικαθιστά το εμφανιζόμενο όνομα που λαμβάνεται από τον χρήστη. **/
    fromUserDisplayName?: string
    tags?: string[]
    media?: FeedPostMediaItem[]
    links?: FeedPostLink[]
    meta?: Record<string, string>
}
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Απόκρισης Δημιουργίας FeedPost'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPostPostResponse {
    status: 'success' | 'failed'
    /** Συμπεριλαμβάνεται σε περίπτωση αποτυχίας. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-from-user-id' | 'invalid-user' | 'broadcast-id-too-long' | 'spam-blocked' | 'user-rate-limited' | 'internal'
    /** Συμπεριλαμβάνεται σε περίπτωση αποτυχίας. **/
    reason?: string
    feedPost?: FeedPost; // We return the complete created post on success.
}
[inline-code-end]