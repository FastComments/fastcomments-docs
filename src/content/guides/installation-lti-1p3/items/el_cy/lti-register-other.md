#### Sakai

Το Sakai υποστηρίζει το LTI 1.3 Dynamic Registration σε εκδόσεις με το LTI Advantage. Από την Περιοχή Διαχείρισης:

1. Συνδεθείτε ως διαχειριστής Sakai και ανοίξτε την **Περιοχή Διαχείρισης**.
2. Επιλέξτε **Εξωτερικά Εργαλεία** > **Εγκατάσταση Εργαλείου LTI 1.3**.
3. Επικολλήστε το URL εγγραφής του FastComments και υποβάλετε.
4. Εγκρίνετε το εργαλείο όταν ολοκληρωθεί η διαδικασία χειραψίας.

Το εργαλείο εμφανίζεται τότε κάτω από τα **Εξωτερικά Εργαλεία** και μπορεί να προστεθεί σε ιστότοπους από τους διαχειριστές τους.

#### Schoology

Οι εγκαταστάσεις Schoology Enterprise υποστηρίζουν το LTI 1.3, αλλά η διαθεσιμότητα της Δυναμικής Εγγραφής διαφέρει ανά εγκατάσταση. Επικοινωνήστε με τον/την υπεύθυνο λογαριασμού Schoology.

Αν η Δυναμική Εγγραφή δεν είναι διαθέσιμη στην εγκατάσταση Schoology σας, θα χρειαστεί να ρυθμίσετε την ενσωμάτωση χειροκίνητα χρησιμοποιώντας αυτά τα endpoints:

- **OIDC Login URL**: `https://fastcomments.com/lti/v1p3/login`
- **Target Link URL**: `https://fastcomments.com/lti/v1p3/launch`
- **Public Keyset URL (JWKS)**: `https://fastcomments.com/lti/v1p3/jwks`
- **Redirect URLs**: `https://fastcomments.com/lti/v1p3/launch`

Αφού το Schoology σας δώσει ένα Client ID και ένα Deployment ID, επικοινωνήστε με την υποστήριξη FastComments για να καταχωρηθεί η διαμόρφωση στον tenant σας.

#### Other LTI 1.3 Platforms

Οποιοδήποτε LMS ακολουθεί το πρότυπο IMS LTI 1.3 Advantage θα πρέπει να λειτουργεί με το ίδιο URL εγγραφής. Αναζητήστε μια ρύθμιση με την ετικέτα "Dynamic Registration", "Tool Registration URL", "Tool initiation registration endpoint" ή παρόμοια.

Αν η πλατφόρμα σας υποστηρίζει μόνο χειροκίνητη ρύθμιση LTI 1.3, χρησιμοποιήστε τα τέσσερα endpoints που αναφέρονται στην ενότητα Schoology παραπάνω και επικοινωνήστε με την υποστήριξη για να ολοκληρώσετε.