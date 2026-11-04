[api-resource-header-start name = 'Comment'; route = 'GET /api/v1/comments'; creditsCost = 1; api-resource-header-end]

Αυτό το API χρησιμοποιείται για τη λήψη σχολίων προς εμφάνιση σε έναν χρήστη. Για παράδειγμα, φιλτράρει αυτόματα
τα μη εγκεκριμένα σχόλια ή τα spam.

### Σελιδοποίηση

Η σελιδοποίηση μπορεί να γίνει με έναν από δύο τρόπους, ανάλογα με τις απαιτήσεις απόδοσης και τη χρήση:

1. Ταχύτερο: **Προϋπολογισμένη Σελιδοποίηση (Precalculated Pagination)**:
   1. Έτσι λειτουργεί το FastComments όταν χρησιμοποιείτε τα έτοιμα widgets και clients μας.
   2. Το πάτημα του "επόμενο" απλώς αυξάνει τον αριθμό της σελίδας.
   3. Μπορείτε να το φανταστείτε ως ανάκτηση από ένα key-value store.
   4. Με αυτόν τον τρόπο, απλώς ορίστε μια παράμετρο `page` ξεκινώντας από `0` και μια κατεύθυνση ταξινόμησης ως `direction`.
   5. Τα μεγέθη σελίδας μπορούν να προσαρμοστούν μέσω κανόνων προσαρμογής.
2. Πιο ευέλικτο: **Ευέλικτη Σελιδοποίηση (Flexible Pagination)**:
   1. Με αυτόν τον τρόπο μπορείτε να ορίσετε προσαρμοσμένες παραμέτρους `limit` και `skip`. Μην περάσετε `page`.
   2. Υποστηρίζεται επίσης η ταξινόμηση `direction`.
   3. Το `limit` είναι το συνολικό πλήθος που θα επιστραφεί μετά την εφαρμογή του `skip`.
      - Παράδειγμα: ορίστε `skip = 200, limit = 100` όταν `page size = 100` και `page = 2`.
   4. Τα θυγατρικά σχόλια μετράνε ακόμα στη σελιδοποίηση. Μπορείτε να το παρακάμψετε χρησιμοποιώντας την επιλογή `asTree`.
      - Μπορείτε να σελιδοποιήσετε τα θυγατρικά μέσω `limitChildren` και `skipChildren`.
      - Μπορείτε να περιορίσετε το βάθος των νημάτων που επιστρέφονται μέσω `maxTreeDepth`.

### Νήματα (Threads)

1. Όταν χρησιμοποιείτε `Precalculated Pagination`, τα σχόλια ομαδοποιούνται ανά *σελίδα* και τα σχόλια στα νήματα επηρεάζουν τη συνολική σελίδα.
   1. Με αυτόν τον τρόπο, τα νήματα μπορούν να προσδιοριστούν στον client με βάση το `parentId`.
   2. Για παράδειγμα, με μια σελίδα που έχει ένα σχόλιο πρώτου επιπέδου και 29 απαντήσεις, και με ορισμό `page=0` στο API - θα λάβετε μόνο το σχόλιο πρώτου επιπέδου και τα 29 θυγατρικά.
2. Όταν χρησιμοποιείτε `Flexible Pagination`, μπορείτε να ορίσετε μια παράμετρο `parentId`.
   1. Ορίστε το σε null για να λάβετε μόνο σχόλια πρώτου επιπέδου.
   2. Στη συνέχεια, για να δείτε τα νήματα, καλέστε ξανά το API περνώντας `parentId`.
   3. Μια συνηθισμένη λύση είναι να κάνετε ένα API call για τα σχόλια πρώτου επιπέδου και στη συνέχεια παράλληλα API calls για τη λήψη σχολίων των θυγατρικών κάθε σχολίου.
3. __ΝΕΟ Από Φεβρουάριο 2023!__ Ανάκτηση ως δέντρο χρησιμοποιώντας `&asTree=true`.
   1. Μπορείτε να το φανταστείτε ως `Flexible Pagination as a Tree`.
   2. Μόνο τα σχόλια πρώτου επιπέδου μετράνε στη σελιδοποίηση.
   3. Ορίστε `parentId=null` για να ξεκινήσει το δέντρο από τη ρίζα (πρέπει να ορίσετε `parentId`).
   4. Ορίστε `skip` και `limit` για τη σελιδοποίηση.
   5. Ορίστε `asTree` σε `true`.
   6. Το κόστος σε credits αυξάνεται κατά `2x`, καθώς το backend μας πρέπει να κάνει πολύ περισσότερη δουλειά σε αυτό το σενάριο.
   7. Ορίστε `maxTreeDepth`, `limitChildren`, και `skipChildren` όπως επιθυμείτε.

### Εξήγηση των Δέντρων

Όταν χρησιμοποιείτε `asTree`, μπορεί να είναι δύσκολο να συλλογιστείτε τη σελιδοποίηση. Ορίστε ένα χρήσιμο διάγραμμα:

<div class="screenshot white-bg">
    <div class="title">Διάγραμμα Σελιδοποίησης Δέντρου</div>
    <img class="screenshot-image" src="/images/fastcomments-comments-api-tree.png" alt="Διάγραμμα Σελιδοποίησης Δέντρου" />
</div>

### Ανάκτηση Σχολίων στο Πλαίσιο ενός Χρήστη

Το API `/comments` μπορεί να χρησιμοποιηθεί σε δύο πλαίσια, για διαφορετικές περιπτώσεις χρήσης:

- Για την επιστροφή σχολίων ταξινομημένων και ετικετοποιημένων με πληροφορίες για τη δημιουργία του δικού σας client.
  - Σε αυτή την περίπτωση, ορίστε μια παράμετρο query `contextUserId`.
- Για την ανάκτηση σχολίων από το backend σας για προσαρμοσμένες ενσωματώσεις.
  - Η πλατφόρμα θα προεπιλέξει αυτό χωρίς `contextUserId`. 

[inline-code-attrs-start title = 'Σελιδοποίηση Σχολίων με Προϋπολογισμό'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&page=0&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR'
[inline-code-end]

[inline-code-attrs-start title = 'Ευέλικτη Σελιδοποίηση Σχολίων'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10'
[inline-code-end]

[inline-code-attrs-start title = 'Ευέλικτη Σελιδοποίηση Σχολίων σε Πλαίσιο Χρήστη'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id'
[inline-code-end]

[inline-code-attrs-start title = 'Ευέλικτη Σελιδοποίηση Σχολίων σε Πλαίσιο Χρήστη Μόνο για Σχόλια Πρώτου Επιπέδου'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null'
[inline-code-end]

### Λήψη Σχολίων ως Δέντρο

Είναι δυνατή η λήψη των σχολίων ως δέντρο, με τη σελιδοποίηση να μετράει μόνο τα σχόλια πρώτου επιπέδου.

[inline-code-attrs-start title = 'Σχόλια Ως Δέντρο σε Πλαίσιο Χρήστη'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true'
[inline-code-end]

Θέλετε να λάβετε μόνο τα σχόλια πρώτου επιπέδου και τα άμεσα θυγατρικά; Ορίστε ένας τρόπος:

[inline-code-attrs-start title = 'Σχόλια Ως Δέντρο με Μέγιστο Βάθος'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&maxTreeDepth=1&limitChildren=10'
[inline-code-end]

Ωστόσο, στο UI σας ίσως χρειάζεται να γνωρίζετε αν πρέπει να εμφανιστεί ένα κουμπί "εμφάνιση απαντήσεων" σε
κάθε σχόλιο. Κατά την ανάκτηση σχολίων ως δέντρο υπάρχει μια ιδιότητα `hasChildren` που επισυνάπτεται
στα σχόλια όταν ισχύει.

### Λήψη Σχολίων ως Δέντρο, Αναζήτηση με Hashtag

Είναι δυνατή η αναζήτηση με hashtag μέσω του API, σε ολόκληρο το tenant σας (όχι περιορισμένο σε μία σελίδα ή `urlId`).

Σε αυτό το παράδειγμα, παραλείπουμε το `urlId`, και αναζητούμε με πολλαπλά hashtags. Το API θα επιστρέψει μόνο σχόλια που έχουν όλα τα ζητούμενα hashtags.

[inline-code-attrs-start title = 'Σχόλια Ως Δέντρο σε Πλαίσιο Χρήστη, Με Hashtag'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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
    /** Το urlId (URL σελίδας ή id άρθρου) με το οποίο σχετίζονται τα σχόλια. **/
    urlId?: string
    /** Περιορίστε τα σχόλια που επιστρέφονται σε αυτόν τον χρήστη. **/
    userId?: string
    /** Χρησιμοποιήστε αυτό για αναζήτηση με hashtag. Για να εμβαθύνετε στην τομή πολλαπλών hashtags, κάντε &hashTag=a&hashTag=b. **/
    hashTag?: string
    /** Η κατεύθυνση ταξινόμησης. Προεπιλογή είναι MR (Most Relevant). Άλλες επιλογές είναι OF (Oldest First) και NF (Newest First). **/
    direction?: 'MR' | 'OF' | 'NF'
    /** Προϋπολογισμένη Σελιδοποίηση: Η σελίδα προς ανάκτηση, ξεκινώντας από 0. Περάστε -1 για όλα τα σχόλια (έως 250). **/
    page?: number
    /** Ευέλικτη Σελιδοποίηση: Πόσα σχόλια πρέπει να επιστρέψουμε; **/
    limit?: number
    /** Ευέλικτη Σελιδοποίηση: Πόσα θυγατρικά σχόλια πρέπει να επιστρέψουμε για κάθε γονέα; **/
    limitChildren?: number
    /** Ευέλικτη Σελιδοποίηση: Πόσα σχόλια πρέπει να παραλείψουμε; **/
    skip?: number
    /** Ευέλικτη Σελιδοποίηση: Πόσα θυγατρικά σχόλια πρέπει να παραλείψουμε για κάθε γονέα; **/
    skipChildren?: number
    /** Για τον προσδιορισμό μπλοκαρισμένων και επισημασμένων σχολίων. **/
    contextUserId?: string
    /** Για τον προσδιορισμό μπλοκαρισμένων και επισημασμένων σχολίων. **/
    anonUserId?: string
    /** Για ανάκτηση θυγατρικών σχολίων. **/
    parentId?: string
    /** Για ανάκτηση ως δέντρο. **/
    asTree?: boolean
    /** Πόσο βαθιά στο δέντρο πρέπει να επιστρέψουμε δεδομένα; Το 0 δεν επιστρέφει θυγατρικά. Το 1 επιστρέφει τα άμεσα θυγατρικά, κ.ο.κ. **/
    maxTreeDepth?: number
}
[inline-code-end]

### Η Απάντηση

[inline-code-attrs-start title = 'Δομή Απάντησης Σχολίων'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsResponse {
    status: 'success' | 'failed'
    /** Συμπεριλαμβάνεται σε αποτυχία. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'missing-date' | 'unauthorized-page' | 'invalid-pagination-request' | 'invalid-limit' | 'invalid-limit-children' | 'invalid-skip' | 'invalid-skip-children' | 'invalid-max-tree-depth'
    /** Συμπεριλαμβάνεται σε αποτυχία. **/
    reason?: string
    /** Τα σχόλια! **/
    comments: Comment[]
}
[inline-code-end]

### Χρήσιμες Συμβουλές

#### URL ID

Πιθανότατα θέλετε να χρησιμοποιήσετε το API `Comment` με την παράμετρο `urlId`. Μπορείτε πρώτα να καλέσετε το API `Pages`, για να δείτε πώς μοιάζουν οι διαθέσιμες τιμές `urlId`. 

#### Ανώνυμες Ενέργειες

Για ανώνυμο σχολιασμό πιθανότατα θέλετε να περάσετε `anonUserId` κατά την ανάκτηση σχολίων, και κατά την επισημείωση και τον μπλοκάρισμα.

(!) Αυτό απαιτείται από πολλά app stores, καθώς οι χρήστες πρέπει να μπορούν να επισημαίνουν περιεχόμενο που δημιουργήθηκε από χρήστες και μπορούν να δουν, ακόμα κι αν δεν είναι συνδεδεμένοι. Η μη τήρηση αυτού μπορεί να προκαλέσει την αφαίρεση της εφαρμογής σας από το εν λόγω store.

#### Σχόλια που δεν Επιστρέφονται

Ελέγξτε ότι τα σχόλιά σας είναι εγκεκριμένα και δεν είναι spam.