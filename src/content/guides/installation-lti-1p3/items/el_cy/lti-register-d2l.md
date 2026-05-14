D2L Brightspace εκθέτει τη Δυναμική Εγγραφή μέσω της διαχειριστικής διεπαφής LTI Advantage. Θα χρειαστείτε πρόσβαση διαχειριστή.

#### Open the Registration Screen

1. Συνδεθείτε στο Brightspace σας ως διαχειριστής.
2. Μεταβείτε σε **Admin Tools** > **Manage Extensibility** > **LTI Advantage**.
3. Κάντε κλικ στο **Register Tool**. (Η άμεση διεύθυνση URL είναι `https://<your-brightspace-host>/d2l/le/ltiadvantage/registrations/create`.)

#### Paste the URL

Θα δείτε μια φόρμα εγγραφής. Το βασικό πεδίο είναι **Tool initiation registration endpoint** (ορισμένες εκδόσεις του Brightspace το ονομάζουν "Tool Initiation Registration URL").

Επικολλήστε το FastComments registration URL (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">λάβετε το εδώ</a>) σε αυτό το πεδίο. Αφήστε τα υπόλοιπα πεδία κενά - συμπληρώνονται αυτόματα από το FastComments κατά τη διάρκεια της χειραψίας εγγραφής.

Κάντε κλικ στο **Register**.

#### Approve the Tool

Το Brightspace ανοίγει ένα αναδυόμενο παράθυρο που επικοινωνεί με το FastComments, ανταλλάσσει κλειδιά και εμφανίζει μια οθόνη επιβεβαίωσης. Το αναδυόμενο κλείνει μόνο του όταν ολοκληρωθεί η εγγραφή.

Το νέο εργαλείο εμφανίζεται στη λίστα εργαλείων LTI Advantage. Από προεπιλογή το Brightspace σημειώνει τα νέα εργαλεία ως **disabled** - μετακινηθείτε τον διακόπτη σε **enabled** ώστε τα μαθήματά σας να μπορούν να το χρησιμοποιήσουν.

#### Add a Deployment

Στο Brightspace, τα εργαλεία LTI χρειάζονται ένα **deployment** πριν μπορέσουν να χρησιμοποιηθούν στα μαθήματα:

1. Ανοίξτε το νεοεγγεγραμμένο εργαλείο FastComments.
2. Κάντε κλικ στο **View Deployments** > **New Deployment**.
3. Δώστε ένα όνομα στο deployment (π.χ. "FastComments - All Courses"), επιλέξτε τις μονάδες οργανισμού στις οποίες θα είναι διαθέσιμο και αποθηκεύστε.

Μετά την πρώτη εκκίνηση μέσω αυτού του deployment, το FastComments καρφώνει το `deployment_id` στην εγγραφή διαμόρφωσής του - οι επακόλουθες εκκινήσεις από διαφορετικό deployment κάτω από τον ίδιο client θα απορριφθούν εκτός αν εγγραφείτε ξανά.