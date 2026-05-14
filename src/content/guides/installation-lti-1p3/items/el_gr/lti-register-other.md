#### Sakai

Το Sakai υποστηρίζει το LTI 1.3 Dynamic Registration σε εκδόσεις με LTI Advantage. Από τον **Χώρο εργασίας διαχείρισης**:

1. Συνδεθείτε ως διαχειριστής Sakai και ανοίξτε τον **Χώρο εργασίας διαχείρισης**.
2. Επιλέξτε **Εξωτερικά Εργαλεία** > **Εγκατάσταση εργαλείου LTI 1.3**.
3. Επικολλήστε το FastComments registration URL και υποβάλετε.
4. Εγκρίνετε το εργαλείο όταν ολοκληρωθεί το handshake.

Το εργαλείο εμφανίζεται στη συνέχεια κάτω από τα **Εξωτερικά Εργαλεία** και μπορεί να προστεθεί σε ιστότοπους από τους διαχειριστές τους.

#### Schoology

Οι Enterprise εγκαταστάσεις του Schoology υποστηρίζουν LTI 1.3 αλλά η διαθεσιμότητα του Dynamic Registration διαφέρει ανά εγκατάσταση. Ελέγξτε με τον/την διαχειριστή λογαριασμού Schoology σας.

Αν το Dynamic Registration δεν είναι διαθέσιμο στην εγκατάσταση Schoology σας, θα χρειαστεί να διαμορφώσετε την ενσωμάτωση χειροκίνητα χρησιμοποιώντας αυτά τα endpoints:

- **OIDC Login URL**: `https://fastcomments.com/lti/v1p3/login`
- **Target Link URL**: `https://fastcomments.com/lti/v1p3/launch`
- **Public Keyset URL (JWKS)**: `https://fastcomments.com/lti/v1p3/jwks`
- **Redirect URLs**: `https://fastcomments.com/lti/v1p3/launch`

Αφού το Schoology σας παρέξει ένα Client ID και Deployment ID, επικοινωνήστε με την υποστήριξη FastComments για να καταχωρήσετε τη διαμόρφωση στον tenant σας.

#### Other LTI 1.3 Platforms

Οποιοδήποτε LMS ακολουθεί το πρότυπο IMS LTI 1.3 Advantage θα πρέπει να λειτουργεί με το ίδιο registration URL. Ψάξτε για μια ρύθμιση με την ετικέτα "Dynamic Registration", "Tool Registration URL", "Tool initiation registration endpoint", ή παρόμοια.

Αν η πλατφόρμα σας υποστηρίζει μόνο χειροκίνητη ρύθμιση LTI 1.3, χρησιμοποιήστε τα τέσσερα endpoints που αναφέρονται στην ενότητα Schoology παραπάνω και επικοινωνήστε με την υποστήριξη για να ολοκληρώσετε.