#### Sakai

Το Sakai υποστηρίζει LTI 1.3 Dynamic Registration σε εκδόσεις με LTI Advantage. Από το Administration Workspace:

1. Συνδεθείτε ως διαχειριστής Sakai και ανοίξτε το **Administration Workspace**.
2. Επιλέξτε **External Tools** > **Install LTI 1.3 Tool**.
3. Επικολλήστε το URL εγγραφής του FastComments (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">βρείτε το εδώ</a>) και υποβάλετε.
4. Εγκρίνετε το εργαλείο όταν ολοκληρωθεί η διαδικασία χειραψίας.

Το εργαλείο εμφανίζεται στη συνέχεια κάτω από **External Tools** και μπορεί να προστεθεί σε ιστότοπους από τους συντηρητές τους.

#### Schoology

Οι περιπτώσεις Schoology Enterprise υποστηρίζουν LTI 1.3, αλλά η διαθεσιμότητα του Dynamic Registration ποικίλλει ανά deployment. Ελέγξτε με τον υπεύθυνο λογαριασμού σας στο Schoology.

Αν το Dynamic Registration δεν είναι διαθέσιμο στη στιγμιότυπό σας Schoology, θα χρειαστεί να ρυθμίσετε την ενσωμάτωση χειροκίνητα χρησιμοποιώντας τα εξής endpoints:

- **OIDC Login URL**: `https://fastcomments.com/lti/v1p3/login`
- **Target Link URL**: `https://fastcomments.com/lti/v1p3/launch`
- **Public Keyset URL (JWKS)**: `https://fastcomments.com/lti/v1p3/jwks`
- **Redirect URLs**: `https://fastcomments.com/lti/v1p3/launch`

Αφού το Schoology σας δώσει ένα Client ID και Deployment ID, επικοινωνήστε με την υποστήριξη του FastComments για να καταχωρήσετε τη ρύθμιση στο tenant σας.

#### Άλλες πλατφόρμες LTI 1.3

Οποιοδήποτε LMS ακολουθεί την προδιαγραφή IMS LTI 1.3 Advantage θα πρέπει να λειτουργεί με το ίδιο URL εγγραφής (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">βρείτε το εδώ</a>). Αναζητήστε μια ρύθμιση με ετικέτα "Dynamic Registration", "Tool Registration URL", "Tool initiation registration endpoint" ή παρόμοια.

Αν η πλατφόρμα σας υποστηρίζει μόνο χειροκίνητη ρύθμιση LTI 1.3, χρησιμοποιήστε τα τέσσερα endpoints που αναφέρονται στην ενότητα Schoology παραπάνω και επικοινωνήστε με την υποστήριξη για να ολοκληρώσετε.