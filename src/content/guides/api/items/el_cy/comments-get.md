[api-resource-header-start name = 'Comment'; route = 'GET /api/v1/comments'; creditsCost = 1; api-resource-header-end]

Αυτό το API χρησιμοποιείται για να πάρετε σχόλια για εμφάνιση σε έναν χρήστη. Για παράδειγμα, φιλτράρει αυτόματα
τα μη εγκεκριμένα ή spam σχόλια.

### Σελιδοποίηση

Η σελιδοποίηση μπορεί να γίνει με έναν από δύο τρόπους, ανάλογα με τις απαιτήσεις απόδοσης και την περίπτωση χρήσης:

1. Ταχύτερη: **Προϋπολογισμένη Σελιδοποίηση**:
   1. Έτσι λειτουργεί το FastComments όταν χρησιμοποιείτε τα προκατασκευασμένα widgets και clients μας.
   2. Κάνοντας κλικ στο "επόμενο" απλά αυξάνει τον αριθμό σελίδας.
   3. Μπορείτε να το σκεφτείτε ως ανάκτηση από ένα key-value store.
   4. Με αυτόν τον τρόπο, απλά ορίστε μια παράμετρο `page` ξεκινώντας από `0` και μια κατεύθυνση ταξινόμησης ως `direction`.
   5. Τα μεγέθη σελίδων μπορούν να προσαρμοστούν μέσω κανόνων προσαρμογής.
2. Πιο Ευέλικτη: **Ευέλικτη Σελιδοποίηση**:
   1. Με αυτόν τον τρόπο μπορείτε να ορίσετε προσαρμοσμένες παραμέτρους `limit` και `skip`. Μην περνάτε `page`.
   2. Η κατεύθυνση ταξινόμησης `direction` υποστηρίζεται επίσης.
   3. Το `limit` είναι ο συνολικός αριθμός για επιστροφή μετά την εφαρμογή του `skip`.
      - Παράδειγμα: ορίστε `skip = 200, limit = 100` όταν `page size = 100` και `page = 2`.
   4. Τα θυγατρικά σχόλια εξακολουθούν να μετρούν στη σελιδοποίηση. Μπορείτε να το παρακάμψετε χρησιμοποιώντας την επιλογή `asTree`.
      - Μπορείτε να σελιδοποιήσετε τα παιδιά μέσω `limitChildren` και `skipChildren`.
      - Μπορείτε να περιορίσετε το βάθος των threads που επιστρέφονται μέσω `maxTreeDepth`.

### Threads

1. Όταν χρησιμοποιείτε `Προϋπολογισμένη Σελιδοποίηση`, τα σχόλια ομαδοποιούνται ανά *σελίδα* και τα σχόλια σε threads επηρεάζουν τη συνολική σελίδα.
   1. Με αυτόν τον τρόπο, τα threads μπορούν να καθοριστούν στον client βάσει του `parentId`.
   2. Για παράδειγμα, με μια σελίδα με ένα σχόλιο κορυφαίου επιπέδου, και 29 απαντήσεις, και ρυθμίζοντας `page=0` στο API - θα πάρετε μόνο το σχόλιο κορυφαίου επιπέδου και τα 29 παιδιά.
   3. [Παράδειγμα εικόνας εδώ που απεικονίζει πολλαπλές σελίδες.](https://blog.winricklabs.com/images/fc-pagination02.png)
2. Όταν χρησιμοποιείτε `Ευέλικτη Σελιδοποίηση`, μπορείτε να ορίσετε μια παράμετρο `parentId`.
   1. Ορίστε αυτό σε null για να πάρετε μόνο σχόλια κορυφαίου επιπέδου.
   2. Στη συνέχεια για να δείτε threads, καλέστε ξανά το API και περάστε `parentId`.
   3. Μια συνηθισμένη λύση είναι να κάνετε μια κλήση API για τα σχόλια κορυφαίου επιπέδου και στη συνέχεια να κάνετε παράλληλες κλήσεις API για να πάρετε σχόλια για τα παιδιά κάθε σχολίου.
3. __ΝΕΟ από Φεβ 2023!__ Ανάκτηση ως δέντρο χρησιμοποιώντας `&asTree=true`.
   1. Μπορείτε να το σκεφτείτε ως `Ευέλικτη Σελιδοποίηση ως Δέντρο`.
   2. Μόνο τα σχόλια κορυφαίου επιπέδου μετρούν στη σελιδοποίηση.
   3. Ορίστε `parentId=null` για να ξεκινήσετε το δέντρο από τη ρίζα (πρέπει να ορίσετε `parentId`).
   4. Ορίστε `skip` και `limit` για σελιδοποίηση.
   5. Ορίστε `asTree` σε `true`.
   6. Το κόστος credits αυξάνεται κατά `2x`, καθώς το backend μας πρέπει να κάνει πολύ περισσότερη δουλειά σε αυτό το σενάριο.
   7. Ορίστε `maxTreeDepth`, `limitChildren`, και `skipChildren` όπως επιθυμείτε.

### Εξήγηση Δέντρων

Όταν χρησιμοποιείτε `asTree`, μπορεί να είναι δύσκολο να συλλάβετε τη σελιδοποίηση. Εδώ είναι ένα χρήσιμο γραφικό:

<div class="screenshot white-bg">
    <div class="title">Διάγραμμα Σελιδοποίησης Δέντρου</div>
    <img class="screenshot-image" src="/images/fastcomments-comments-api-tree.png" alt="Διάγραμμα Σελιδοποίησης Δέντρου" />
</div>

### Ανάκτηση Σχολίων στο Πλαίσιο ενός Χρήστη

Το `/comments` API μπορεί να χρησιμοποιηθεί σε δύο πλαίσια, για διαφορετικές περιπτώσεις χρήσης:

- Για επιστροφή σχολίων ταξινομημένων και με πληροφορίες για την κατασκευή του δικού σας client.
  - Σε αυτή την περίπτωση, ορίστε μια παράμετρο query `contextUserId`.
- Για ανάκτηση σχολίων από το backend σας για προσαρμοσμένες ενσωματώσεις.
  - Η πλατφόρμα θα επιλέξει αυτό από προεπιλογή χωρίς `contextUserId`.

[inline-code-attrs-start title = 'Σχόλια Προϋπολογισμένη Σελιδοποίηση'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&page=0&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR'
[inline-code-end]

[inline-code-attrs-start title = 'Σχόλια Ευέλικτη Σελιδοποίηση'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10'
[inline-code-end]

[inline-code-attrs-start title = 'Σχόλια Ευέλικτη Σελιδοποίηση σε Πλαίσιο Χρήστη'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id'
[inline-code-end]

[inline-code-attrs-start title = 'Σχόλια Ευέλικτη Σελιδοποίηση σε Πλαίσιο Χρήστη μόνο για Σχόλια Κορυφαίου Επιπέδου'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null'
[inline-code-end]

### Ανάκτηση Σχολίων ως Δέντρο

Είναι δυνατό να πάρετε τα σχόλια που επιστρέφονται ως δέντρο, με τη σελιδοποίηση να μετρά μόνο τα σχόλια κορυφαίου επιπέδου.

[inline-code-attrs-start title = 'Σχόλια Ως-Δέντρο σε Πλαίσιο Χρήστη'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true'
[inline-code-end]

Θέλετε να πάρετε μόνο τα σχόλια κορυφαίου επιπέδου και τα άμεσα παιδιά; Εδώ είναι ένας τρόπος:

[inline-code-attrs-start title = 'Σχόλια Ως-Δέντρο με Μέγιστο Βάθος'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&maxTreeDepth=1&limitChildren=10'
[inline-code-end]

Ωστόσο, στο UI σας μπορεί να χρειαστεί να γνωρίζετε αν θα εμφανίσετε ένα κουμπί "εμφάνιση απαντήσεων" σε
κάθε σχόλιο. Όταν ανακτάτε σχόλια μέσω δέντρου υπάρχει μια ιδιότητα `hasChildren` που προστίθεται
στα σχόλια όταν ισχύει.

### Ανάκτηση Σχολίων ως Δέντρο, Αναζήτηση με Hash Tag

Είναι δυνατή η αναζήτηση με hashtag χρησιμοποιώντας το API, σε ολόκληρο τον tenant σας (όχι περιορισμένη σε μία σελίδα, ή `urlId`).

Σε αυτό το παράδειγμα, παραλείπουμε το `urlId`, και αναζητούμε με πολλαπλά hashtags. Το API θα επιστρέψει μόνο σχόλια που έχουν όλα τα ζητούμενα hashtags.

[inline-code-attrs-start title = 'Σχόλια Ως-Δέντρο σε Πλαίσιο Χρήστη, με Hash Tag'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&hashTag=TestTag&hashTag=OtherTestTag'
[inline-code-end]

### Όλες οι Παράμετροι Αιτήματος

[inline-code-attrs-start title = 'Δομή Αιτήματος Σχολίων'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** The urlId (page url, or article id) the comments are associated with. **/
    urlId?: string
    /** Limit the comments returned by this user. **/
    userId?: string
    /** Use this to search by hashtag. To drill down to the intersection of multiple hashtags, do &hashTag=a&hashTag=b. **/
    hashTag?: string
    /** The sort direction. Default is MR (Most Relevant). Other options are OF (Oldest First) and NF (Newest First). **/
    direction?: 'MR' | 'OF' | 'NF'
    /** Precalculated Pagination: The page to fetch, starting with 0. Pass -1 for all comments (up to 250). **/
    page?: number
    /** Flexible Pagination: How many comments should we return? **/
    limit?: number
    /** Flexible Pagination: How many child comments should we return for each parent? **/
    limitChildren?: number
    /** Flexible Pagination: How many comments should we skip? **/
    skip?: number
    /** Flexible Pagination: How many child comments should we skip for each parent? **/
    skipChildren?: number
    /** For determining blocked and flagged comments. **/
    contextUserId?: string
    /** For determining blocked and flagged comments. **/
    anonUserId?: string
    /** For fetching child comments. **/
    parentId?: string
    /** For fetching as a tree. **/
    asTree?: boolean
    /** How far into the tree should we return data? 0 returns no children. 1 returns immediate children, etc. **/
    maxTreeDepth?: number
}
[inline-code-end]

### Η Απάντηση

[inline-code-attrs-start title = 'Δομή Απάντησης Σχολίων'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'missing-date' | 'unauthorized-page' | 'invalid-pagination-request' | 'invalid-limit' | 'invalid-limit-children' | 'invalid-skip' | 'invalid-skip-children' | 'invalid-max-tree-depth'
    /** Included on failure. **/
    reason?: string
    /** The comments! **/
    comments: Comment[]
}
[inline-code-end]

### Χρήσιμες Συμβουλές

#### URL ID

Πιθανότατα θέλετε να χρησιμοποιήσετε το `Comment` API με την παράμετρο `urlId`. Μπορείτε να καλέσετε πρώτα το `Pages` API, για να δείτε πώς μοιάζουν οι διαθέσιμες τιμές `urlId`.

#### Ανώνυμες Ενέργειες

Για ανώνυμο σχολιασμό πιθανότατα θέλετε να περνάτε `anonUserId` όταν ανακτάτε σχόλια, και όταν εκτελείτε επισήμανση και αποκλεισμό.

(!) Αυτό απαιτείται για πολλά app stores καθώς οι χρήστες πρέπει να μπορούν να επισημάνουν περιεχόμενο που δημιουργήθηκε από χρήστες που μπορούν να δουν, ακόμα κι αν δεν είναι συνδεδεμένοι. Η μη συμμόρφωση μπορεί να προκαλέσει την αφαίρεση της εφαρμογής σας από το εν λόγω store.

#### Σχόλια που δεν Επιστρέφονται

Ελέγξτε ότι τα σχόλιά σας είναι εγκεκριμένα, και δεν είναι spam.
