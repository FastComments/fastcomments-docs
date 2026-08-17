[api-resource-header-start name = 'Comment'; route = 'GET /api/v1/comments'; creditsCost = 1; api-resource-header-end]

Αυτό το API χρησιμοποιείται για την λήψη σχολίων ώστε να εμφανιστούν σε έναν χρήστη. Για παράδειγμα, φιλτράρει αυτόματα τα μη εγκεκριμένα ή ανεπιθύμητα σχόλια.

### Pagination

Η σελιδοποίηση μπορεί να γίνει με έναν από τους δύο τρόπους, ανάλογα με τις απαιτήσεις απόδοσης και τη χρήση:

1. **Γρηγορότερο: Precalculated Pagination**:
   1. Αυτή είναι η λειτουργία του FastComments όταν χρησιμοποιείτε τα προ‑κατασκευασμένα widget και πελάτες μας.
   2. Κάνοντας κλικ στο "next" απλώς αυξάνει τον αριθμό σελίδας.
   3. Μπορείτε να το σκεφτείτε ως ανάκτηση από αποθήκη κλειδιού‑τιμής.
   4. Με αυτόν τον τρόπο, απλώς ορίστε μια παράμετρο `page` που ξεκινά από `0` και μια κατεύθυνση ταξινόμησης ως `direction`.
   5. Τα μεγέθη σελίδας μπορούν να προσαρμοστούν μέσω κανόνων προσαρμογής.
2. **Πιο Ευέλικτο: Flexible Pagination**:
   1. Με αυτόν τον τρόπο μπορείτε να ορίσετε προσαρμοσμένες παραμέτρους `limit` και `skip`. Μην περάσετε `page`.
   2. Η ταξινόμηση `direction` υποστηρίζεται επίσης.
   3. `limit` είναι ο συνολικός αριθμός που θα επιστραφεί μετά την εφαρμογή του `skip`.
      - Παράδειγμα: ορίστε `skip = 200, limit = 100` όταν `page size = 100` και `page = 2`.
   4. Τα σχόλια παιδιών εξακολουθούν να υπολογίζονται στη σελιδοποίηση. Μπορείτε να το παρακάμψετε χρησιμοποιώντας την επιλογή `asTree`.
      - Μπορείτε να σελιδοποιήσετε τα παιδιά μέσω `limitChildren` και `skipChildren`.
      - Μπορείτε να περιορίσετε το βάθος των νήματων που επιστρέφονται μέσω `maxTreeDepth`.

### Threads

1. Όταν χρησιμοποιείται η `Precalculated Pagination`, τα σχόλια ομαδοποιούνται ανά *σελίδα* και τα σχόλια στα νήματα επηρεάζουν τη συνολική σελίδα.
   1. Με αυτόν τον τρόπο, τα νήματα μπορούν να προσδιοριστούν από τον πελάτη βάσει του `parentId`.
   2. Για παράδειγμα, με μια σελίδα που έχει ένα σχόλιο κορυφής και 29 απαντήσεις, και ορίζοντας `page=0` στο API – θα λάβετε μόνο το σχόλιο κορυφής και τα 29 παιδιά.
2. Όταν χρησιμοποιείται η `Flexible Pagination`, μπορείτε να ορίσετε μια παράμετρο `parentId`.
   1. Ορίστε το σε null για να λάβετε μόνο σχόλια κορυφής.
   2. Στη συνέχεια, για να δείτε τα νήματα, καλέστε ξανά το API και περάστε το `parentId`.
   3. Μια κοινή λύση είναι να κάνετε ένα αίτημα API για τα σχόλια κορυφής και στη συνέχεια παράλληλα αιτήματα για να λάβετε τα σχόλια των παιδιών κάθε σχολίου.
3. __ΝΕΟ Από Φεβ 2023!__ Λάβετε ως δέντρο χρησιμοποιώντας `&asTree=true`.
   1. Μπορείτε να το σκεφτείτε ως `Flexible Pagination ως Δέντρο`.
   2. Μόνο τα σχόλια κορυφής υπολογίζονται στη σελιδοποίηση.
   3. Ορίστε `parentId=null` για να ξεκινήσετε το δέντρο από τη ρίζα (πρέπει να ορίσετε το `parentId`).
   4. Ορίστε `skip` και `limit` για σελιδοποίηση.
   5. Ορίστε `asTree` σε `true`.
   6. Το κόστος πίστωσης αυξάνεται κατά `2x`, καθώς το backend μας πρέπει να κάνει πολύ περισσότερη δουλειά σε αυτό το σενάριο.
   7. Ορίστε `maxTreeDepth`, `limitChildren` και `skipChildren` όπως επιθυμείτε.

### Trees Explained

Όταν χρησιμοποιείται το `asTree`, μπορεί να είναι δύσκολο να κατανοηθεί η σελιδοποίηση. Εδώ είναι ένα χρήσιμο γραφικό:

<div class="screenshot white-bg">
    <div class="title">Διάγραμμα Σελιδοποίησης Δέντρου</div>
    <img class="screenshot-image" src="/images/fastcomments-comments-api-tree.png" alt="Διάγραμμα Σελιδοποίησης Δέντρου" />
</div>

### Fetching Comments in The Context of a User

Το API `/comments` μπορεί να χρησιμοποιηθεί σε δύο πλαίσια, για διαφορετικές περιπτώσεις χρήσης:

- Για επιστροφή σχολίων ταξινομημένων και επισημασμένων με πληροφορίες για την κατασκευή του δικού σας πελάτη.
  - Σε αυτή την περίπτωση, ορίστε μια παράμετρο ερωτήματος `contextUserId`.
- Για λήψη σχολίων από το backend σας για προσαρμοσμένες ενσωματώσεις.
  - Η πλατφόρμα θα χρησιμοποιήσει αυτήν την επιλογή χωρίς `contextUserId`.

[inline-code-attrs-start title = 'Σχόλια Προκαθορισμένη Σελιδοποίηση'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Σχόλια Ευέλικτη Σελιδοποίηση σε Πλαίσιο Χρήστη μόνο για Σχόλια Κορυφής'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null'
[inline-code-end]

### Get Comments as a Tree

Είναι δυνατόν να λάβετε τα σχόλια ως δέντρο, με τη σελιδοποίηση να μετρά μόνο τα σχόλια κορυφής.

[inline-code-attrs-start title = 'Σχόλια Ως Δέντρο σε Πλαίσιο Χρήστη'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true'
[inline-code-end]

Θέλετε να λάβετε μόνο τα σχόλια κορυφής και τα άμεσα παιδιά; Εδώ είναι ένας τρόπος:

[inline-code-attrs-start title = 'Σχόλια Ως Δέντρο με Μέγιστο Βάθος'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&maxTreeDepth=1&limitChildren=10'
[inline-code-end]

Ωστόσο, στο UI σας μπορεί να χρειαστεί να γνωρίζετε αν θα εμφανίσετε ένα κουμπί "εμφάνιση απαντήσεων" σε κάθε σχόλιο. Όταν λαμβάνετε σχόλια μέσω δέντρου, υπάρχει η ιδιότητα `hasChildren` που επισημαίνεται στα σχόλια όταν είναι εφαρμόσιμη.

### Get Comments as a Tree, Searching by Hash Tag

Είναι δυνατόν να αναζητήσετε με hashtag χρησιμοποιώντας το API, σε όλο το ενοικιαστή σας (δεν περιορίζεται σε μία σελίδα ή `urlId`).

Σε αυτό το παράδειγμα, παραλείπουμε το `urlId` και αναζητούμε με πολλαπλές ετικέτες hashtag. Το API θα επιστρέψει μόνο τα σχόλια που έχουν όλες τις ζητούμενες ετικέτες.

[inline-code-attrs-start title = 'Σχόλια Ως Δέντρο σε Πλαίσιο Χρήστη, Κατά Ετικέτα Hash'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&hashTag=TestTag&hashTag=OtherTestTag'
[inline-code-end]

### All Request Params

[inline-code-attrs-start title = 'Δομή Αιτήματος Σχολίων'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** Το urlId (URL σελίδας ή ID άρθρου) με το οποίο σχετίζονται τα σχόλια. **/
    urlId?: string
    /** Περιορίστε τα σχόλια που επιστρέφονται από αυτόν τον χρήστη. **/
    userId?: string
    /** Χρησιμοποιήστε αυτό για αναζήτηση με ετικέτα hashtag. Για να φτάσετε στην τομή πολλαπλών hashtags, χρησιμοποιήστε &hashTag=a&hashTag=b. **/
    hashTag?: string
    /** Η κατεύθυνση ταξινόμησης. Η προεπιλογή είναι MR (Πιο Σχετικό). Άλλες επιλογές είναι OF (Παλαιότερα Πρώτα) και NF (Νεότερα Πρώτα). **/
    direction?: 'MR' | 'OF' | 'NF'
    /** Προκαθορισμένη Σελιδοποίηση: Η σελίδα που θα ληφθεί, ξεκινώντας από 0. Δώστε -1 για όλα τα σχόλια (μέχρι 250). **/
    page?: number
    /** Ευέλικτη Σελιδοποίηση: Πόσα σχόλια πρέπει να επιστρέψουμε; **/
    limit?: number
    /** Ευέλικτη Σελιδοποίηση: Πόσα σχόλια παιδιών πρέπει να επιστρέψουμε για κάθε γονέα; **/
    limitChildren?: number
    /** Ευέλικτη Σελιδοποίηση: Πόσα σχόλια πρέπει να παραλείψουμε; **/
    skip?: number
    /** Ευέλικτη Σελιδοποίηση: Πόσα σχόλια παιδιών πρέπει να παραλείψουμε για κάθε γονέα; **/
    skipChildren?: number
    /** Για τον καθορισμό μπλοκαρισμένων και επισημασμένων σχολίων. **/
    contextUserId?: string
    /** Για τον καθορισμό μπλοκαρισμένων και επισημασμένων σχολίων. **/
    anonUserId?: string
    /** Για την λήψη σχολίων παιδιών. **/
    parentId?: string
    /** Για λήψη ως δέντρο. **/
    asTree?: boolean
    /** Μέχρι ποιο βάθος του δέντρου πρέπει να επιστρέψουμε δεδομένα; 0 δεν επιστρέφει παιδιά. 1 επιστρέφει άμεσα παιδιά, κ.λπ. **/
    maxTreeDepth?: number
}
[inline-code-end]

### The Response

[inline-code-attrs-start title = 'Δομή Απόκρισης Σχολίων'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsResponse {
    status: 'success' | 'failed'
    /** Συμπεριλαμβάνεται σε περίπτωση αποτυχίας. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'missing-date' | 'unauthorized-page' | 'invalid-pagination-request' | 'invalid-limit' | 'invalid-limit-children' | 'invalid-skip' | 'invalid-skip-children' | 'invalid-max-tree-depth'
    /** Συμπεριλαμβάνεται σε περίπτωση αποτυχίας. **/
    reason?: string
    /** Τα σχόλια! **/
    comments: Comment[]
}
[inline-code-end]

### Helpful Tips

#### URL ID

Πιθανώς θέλετε να χρησιμοποιήσετε το API `Comment` με την παράμετρο `urlId`. Μπορείτε πρώτα να καλέσετε το API `Pages`, για να δείτε πώς φαίνονται οι διαθέσιμες τιμές `urlId`.

#### Anonymous Actions

Για ανώνυμη σχολιασμό, πιθανώς θέλετε να περάσετε το `anonUserId` όταν λαμβάνετε σχόλια και όταν εκτελείτε σημαδοποίηση και φραγή.

(!) Αυτό απαιτείται από πολλά app stores, καθώς οι χρήστες πρέπει να μπορούν να σημαδέψουν περιεχόμενο που δημιουργήθηκε από χρήστες και το βλέπουν, ακόμη και αν δεν είναι συνδεδεμένοι. Η μη τήρηση μπορεί να οδηγήσει στην αφαίρεση της εφαρμογής σας από το εν λόγω κατάστημα.

#### Comments Not Being Returned

Βεβαιωθείτε ότι τα σχόλια σας είναι εγκεκριμένα και δεν είναι ανεπιθύμητα.