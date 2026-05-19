Μόλις το FastComments καταχωρηθεί στην πλατφόρμα, οι εκπαιδευτές το προσθέτουν στο περιεχόμενο του μαθήματος χρησιμοποιώντας τις τυπικές ροές εξωτερικών εργαλείων της πλατφόρμας. Αυτή η σελίδα καλύπτει το Sakai 23.x και το Schoology Enterprise.

#### Lock Down Public Access (Recommended)

Από προεπιλογή, τα δεδομένα σχολίων του FastComments είναι αναγνώσιμα δημόσια σε οποιαδήποτε από τις δύο πλατφόρμες. Οποιοσδήποτε μπορεί να μαντέψει το URL ενός thread ή το API endpoint και να δει τα σχόλια, ακόμα και εκτός Sakai ή Schoology. Για τις συζητήσεις του μαθήματος σχεδόν σίγουρα θέλετε να περιορίσετε την προβολή μόνο σε εγγεγραμμένους φοιτητές.

Ανοίξτε την <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">σελίδα προσαρμογής widget</a> και δημιουργήστε έναν κανόνα με **Require SSO To View Comments** ενεργοποιημένο, στη συνέχεια ορίστε το επίπεδο ασφαλείας σε **Secure SSO** ώστε τα threads να μπορούν να φορτωθούν μόνο μέσω της υπογεγραμμένης εκκίνησης LTI.

Δείτε [Protecting Comment Threads With Single-Sign-On](/guide-customizations-and-configuration.html#sso-require-to-view-comments) για τον πλήρη οδηγό, συμπεριλαμβανομένου του τρόπου περιορισμού του κανόνα σε ένα μόνο domain ή σελίδα.

#### Sakai

**1. Add FastComments to a site**

Ο διαχειριστής της τοποθεσίας ενεργοποιεί το εργαλείο ανά τοποθεσία:

1. Ανοίξτε την τοποθεσία και κάντε κλικ στο **Site Info** στην αριστερή πλοήγηση.
2. Κάντε κλικ στο **Manage Tools**.
3. Κάντε κύλιση στη λίστα **External Tools** και ενεργοποιήστε το **FastComments**.
4. Κάντε κλικ στο **Continue**, ελέγξτε τη λίστα εργαλείων και μετά κάντε κλικ στο **Finish**.

Το FastComments εμφανίζεται πλέον ως στοιχείο στην αριστερή πλοήγηση της τοποθεσίας.

**2. Reorder the left-nav entry**

Πηγαίνετε στο **Site Info** > **Tool Order**. Σύρετε το **FastComments** στη θέση που επιθυμείτε και κάντε κλικ στο **Save**. Μπορείτε επίσης να μετονομάσετε την ετικέτα πλοήγησης και να την αποκρύψετε από τους φοιτητές από αυτή την οθόνη.

**3. Embed inline in a Lessons page**

Για να τοποθετήσετε το FastComments απευθείας μέσα σε μια σελίδα Lessons αντί ως αυτόνομο εργαλείο στην αριστερή πλοήγηση:

1. Ανοίξτε το εργαλείο **Lessons** στην τοποθεσία.
2. Κάντε κλικ **Add Content** > **Add External Tool**.
3. Επιλέξτε **FastComments** από τη λίστα.
4. Αν το FastComments διαφήμισε το Deep Linking κατά την καταχώριση, το Sakai ανοίγει τον επιλογέα περιεχομένου του εργαλείου ώστε να μπορείτε να επιλέξετε ή να ονομάσετε το thread. Αν το Deep Linking δεν διαφημίστηκε, το Sakai εισάγει έναν προεπιλεγμένο σύνδεσμο εκκίνησης.
5. Αποθηκεύστε το στοιχείο Lessons.

Κάθε ενσωματωμένη παρουσία έχει το δικό της thread, περιορισμένο σε αυτόν τον resource link.

**4. Permission tweaks for student access**

Το Sakai ελέγχει τις εκκινήσεις εξωτερικών εργαλείων μέσω Realms. Για να επιβεβαιώσετε ότι οι φοιτητές μπορούν να εκκινήσουν το FastComments:

1. Συνδεθείτε ως διαχειριστής Sakai και ανοίξτε **Administration Workspace** > **Realms**.
2. Ανοίξτε το σχετικό realm (για παράδειγμα, `!site.template.course` ή το συγκεκριμένο realm της τοποθεσίας).
3. Επιβεβαιώστε ότι ο ρόλος `access` έχει ενεργοποιημένο το `lti.launch` και ότι οι άδειες ρόλου στην ομάδα **external.tools** έχουν παραχωρηθεί.
4. Αποθηκεύστε το realm.

Για παραμετροποιήσεις σε επίπεδο τοποθεσίας, ο διαχειριστής μπορεί να ρυθμίσει την ορατότητα του εργαλείου ανά ρόλο από **Site Info** > **Tool Order** αποκρύπτοντας ή εμφανίζοντας το FastComments ανά ρόλο.

**5. What students see**

Οι φοιτητές κάνουν κλικ στο στοιχείο FastComments στην αριστερή πλοήγηση (ή σκρολάρουν στο ενσωματωμένο μπλοκ Lessons) και προσγειώνονται απευθείας στην εμφάνιση με threaded σχόλια. Το SSO γίνεται αυτόματα: το Sakai στέλνει την ταυτότητα του χρήστη στην εκκίνηση LTI και το FastComments τους συνδέει με τον λογαριασμό Sakai τους.

Role mapping:

- Sakai `Instructor` -> FastComments moderator
- Sakai `Admin` (admin in Administration Workspace) -> FastComments admin
- Sakai `Student` / `access` -> FastComments commenter

**6. Sakai gotchas**

- **Tool not visible in Manage Tools.** Εάν το FastComments δεν εμφανίζεται στη λίστα External Tools, ο διαχειριστής Sakai πρέπει να ανοίξει το registry του εργαλείου (**Administration Workspace** > **External Tools** > **FastComments**) και να ορίσει το **Stealthed** σε `false`. Τα stealthed εργαλεία είναι κρυμμένα από τον επιλογέα Manage Tools ανά τοποθεσία.
- **Launches breaking in shared-session browsers.** Το portal CSRF token του Sakai συνδέεται με τη συνεδρία του browser. Εάν ένας φοιτητής είναι συνδεδεμένος σε δύο τοποθεσίες Sakai σε διαφορετικές καρτέλες ή έχει παλαιά συνεδρία, η εκκίνηση επιστρέφει 403. Διόρθωση: κλείστε άλλες καρτέλες Sakai, αποσυνδεθείτε, συνδεθείτε ξανά και επανεκκινήστε. Οι διαχειριστές μπορούν επίσης να αυξήσουν το `sakai.csrf.token.cache.ttl` εάν αυτό συμβαίνει σε ολόκληρο το cluster.
- **Frame embedding.** Επιβεβαιώστε ότι το `lti.frameheight` στο `sakai.properties` είναι αρκετά μεγάλο (600 ή υψηλότερο) ώστε το thread σχολίων να μην περικόπτεται μέσα σε μια σελίδα Lessons.

#### Schoology

Το Schoology Enterprise έχει δύο σενάρια εγκατάστασης. Επιβεβαιώστε ποιο ισχύει πριν προσθέσετε το εργαλείο σε ένα μάθημα.

**1. Two installation scenarios**

- **(a) Enterprise-level install.** Ο System Administrator του Schoology εγκατέστησε το FastComments σε επίπεδο οργανισμού και το ανέθεσε σε όλα τα μαθήματα ή σε συγκεκριμένα templates μαθημάτων. Οι εκπαιδευτές παραλείπουν την εγκατάσταση και πηγαίνουν κατευθείαν στο "Add Materials".
- **(b) Instructor self-install.** Ο εκπαιδευτής εγκαθιστά το εργαλείο σε ένα μόνο μάθημα από **Course Options** > **External Tools** > **Install LTI Apps**. Η αυτοεγκατάσταση απαιτεί ο System Administrator να έχει εγκρίνει πρώτα την εφαρμογή FastComments σε επίπεδο οργανισμού.

**2. Add FastComments as a course material**

Μέσα στο μάθημα:

1. Ανοίξτε το μάθημα και μεταβείτε στα **Materials**.
2. Κάντε κλικ **Add Materials** > **Add File/Link/External Tool**.
3. Επιλέξτε **External Tool**.
4. Επιλέξτε **FastComments** από τη λίστα εγγεγραμμένων εργαλείων.
5. Ορίστε ένα **Name** (αυτό είναι που βλέπουν οι φοιτητές στη λίστα υλικών) και μια προαιρετική **Description**.
6. Αφήστε το **Enable Grading** (grade passback) **OFF**. Το FastComments δεν αναφέρει βαθμούς πίσω στο Schoology, οπότε το να ενεργοποιήσετε το grade passback δημιουργεί μια κενή στήλη βαθμολογίου.
7. Κάντε κλικ **Submit**.

Το υλικό εμφανίζεται πλέον στη λίστα υλικών του μαθήματος και ανοίγει το thread FastComments όταν γίνεται κλικ.

**3. Inline embedding via the Rich Text editor**

Αν ο System Administrator ενεργοποίησε το Deep Linking placement για το FastComments κατά την καταχώριση, οι εκπαιδευτές μπορούν να ενσωματώσουν το thread σχολίων μέσα σε οποιοδήποτε πεδίο Rich Text (οδηγίες ανάθεσης, σώματα σελίδας, προτροπές συζήτησης):

1. Ανοίξτε τον Rich Text editor στη σελίδα-στόχο.
2. Κάντε κλικ στο εικονίδιο **External Tool** (κομματάκι παζλ) στη γραμμή εργαλείων.
3. Επιλέξτε **FastComments**.
4. Διαμορφώστε την ενσωμάτωση στο διάλογο deep-linking και κάντε κλικ **Insert**.
5. Αποθηκεύστε τη σελίδα.

Εάν το κουμπί External Tool δεν εμφανίζεται στον Rich Text editor, το Deep Linking είναι απενεργοποιημένο για αυτό το εργαλείο στον συγκεκριμένο tenant. Δείτε τα gotchas παρακάτω.

**4. Visibility and section assignments**

Το Schoology περιορίζει τη διαθεσιμότητα του εργαλείου ανά section μέσω των Course Options:

1. Από το μάθημα, κάντε κλικ **Course Options** > **External Tools**.
2. Για κάθε εγκατεστημένη εφαρμογή LTI, ελέγχετε αν είναι διαθέσιμη σε όλα τα sections του μαθήματος ή σε συγκεκριμένα sections.
3. Για να περιορίσετε το FastComments σε συγκεκριμένα sections, αποεπιλέξτε τα sections που δεν θα πρέπει να βλέπουν το εργαλείο.
4. Η πρόσβαση σε επίπεδο section επίσης καθορίζει ποια sections βλέπουν την καταχώριση **Add Materials** > **External Tool** για το FastComments.

**5. What students see**

Οι φοιτητές κάνουν κλικ στο υλικό FastComments (ή σκρολάρουν μέχρι την ενσωμάτωση) και προσγειώνονται στη threaded συζήτηση. Το SSO γίνεται αυτόματα μέσω της εκκίνησης LTI του Schoology υπό τον λογαριασμό Schoology τους.

Role mapping:

- Schoology `Administrator` -> FastComments admin
- Schoology `Instructor` -> FastComments moderator
- Schoology `Student` -> FastComments commenter

**6. Schoology gotchas**

- **Enterprise-only.** Οι προσωπικοί και δωρεάν λογαριασμοί Schoology δεν μπορούν να εγκαταστήσουν εργαλεία LTI 1.3. Εάν ο tenant σας είναι στη δωρεάν βαθμίδα, η επιλογή **External Tools** απουσιάζει από τα Course Options. Αναβαθμίστε σε Schoology Enterprise για να χρησιμοποιήσετε το FastComments.
- **Deep Linking disabled by tenant default.** Ορισμένοι tenants του Schoology περιορίζουν το Deep Linking placement σε επίπεδο οργανισμού. Όταν συμβαίνει αυτό, οι εκπαιδευτές βλέπουν μόνο τη ροή **Add Materials** > **External Tool** και όχι το κουμπί External Tool στον Rich Text editor. Για να επιτρέψετε την ενσωμάτωση γραμμής, ο System Administrator πηγαίνει σε **System Settings** > **Integration** > **LTI 1.3** > **FastComments** και ενεργοποιεί το placement **Content Item / Deep Linking**, στη συνέχεια αποθηκεύει.
- **Per-section assignment override.** Εάν το FastComments έχει ανατεθεί σε επίπεδο οργανισμού αλλά ο εκπαιδευτής δεν το βλέπει στο **Add Materials**, το section του μαθήματος αποκλείστηκε στην ανάθεση σε επίπεδο οργανισμού. Ζητήστε από τον System Administrator να προσθέσει το section στην ανάθεση της εφαρμογής FastComments.
- **Material name vs. thread identity.** Η μετονομασία του υλικού στο Schoology δεν μεταφέρει το thread σχολίων. Τα threads βασίζονται στο LTI resource link ID, οπότε μια μετονομασία διατηρεί το ίδιο thread· η διαγραφή και επανδημιουργία του υλικού δημιουργεί ένα νέο, κενό thread.