#### Sakai

Το Sakai υποστηρίζει το LTI 1.3 Dynamic Registration στις εκδόσεις με LTI Advantage. Από το **Administration Workspace**:

1. Συνδεθείτε ως διαχειριστής Sakai και ανοίξτε το **Administration Workspace**.
2. Επιλέξτε **External Tools** > **Install LTI 1.3 Tool**.
3. Επικολλήστε το URL εγγραφής του FastComments (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">αποκτήστε το εδώ</a>) και υποβάλετε.
4. Εγκρίνετε το εργαλείο όταν ολοκληρωθεί η διαδικασία χειραψίας.

Το εργαλείο εμφανίζεται τότε κάτω από **External Tools** και μπορεί να προστεθεί σε ιστότοπους από τους συντηρητές τους.

#### Schoology

Οι εγκαταστάσεις Schoology Enterprise υποστηρίζουν το LTI 1.3 αλλά η διαθεσιμότητα του Dynamic Registration διαφέρει ανά ανάπτυξη. Επικοινωνήστε με τον διαχειριστή λογαριασμού Schoology.

Εάν το Dynamic Registration δεν είναι διαθέσιμο στην εγκατάσταση Schoology σας, θα χρειαστεί να διαμορφώσετε την ενσωμάτωση χειροκίνητα χρησιμοποιώντας αυτά τα endpoints:

- **OIDC Login URL**: `https://fastcomments.com/lti/v1p3/login`
- **Target Link URL**: `https://fastcomments.com/lti/v1p3/launch`
- **Public Keyset URL (JWKS)**: `https://fastcomments.com/lti/v1p3/jwks`
- **Redirect URLs**: `https://fastcomments.com/lti/v1p3/launch`

Αφού το Schoology σας δώσει ένα Client ID και Deployment ID, επικοινωνήστε με την υποστήριξη FastComments για να καταχωρήσετε τη ρύθμιση στον tenant σας.

#### Other LTI 1.3 Platforms

Κάθε LMS που ακολουθεί το πρότυπο IMS LTI 1.3 Advantage θα πρέπει να λειτουργεί με το ίδιο URL εγγραφής (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">αποκτήστε το εδώ</a>). Αναζητήστε μια ρύθμιση με την ονομασία "Dynamic Registration", "Tool Registration URL", "Tool initiation registration endpoint" ή παρόμοια.

Εάν η πλατφόρμα σας υποστηρίζει μόνο χειροκίνητη ρύθμιση LTI 1.3, χρησιμοποιήστε τα τέσσερα endpoints που αναφέρονται στην ενότητα Schoology παραπάνω και επικοινωνήστε με την υποστήριξη για να ολοκληρώσετε τη διαδικασία.