Η ενσωμάτωση FastComments LTI 1.3 ακολουθεί την αρχή του ελάχιστου προνομίου: χρησιμοποιεί μόνο τα launch claims που απαιτούνται για να ταυτοποιήσει τον χρήστη, να συσχετίσει τα σχόλια με το σωστό μάθημα και πόρο, και να εφαρμόσει δικαιώματα βάσει ρόλων.

Οι αναθεωρητές ασφάλειας και προμηθειών μπορούν να αντλήσουν απαντήσεις απευθείας από τους παρακάτω πίνακες.

## Στοιχεία Δεδομένων που Λαμβάνονται από το LMS

Every LTI 1.3 launch carries a signed JWT from the LMS. FastComments extracts the following claims from that JWT and uses nothing else:

| Πεδίο | LTI claim | Σκοπός | Απαιτείται | Αποθηκεύεται |
|-------|-----------|--------|------------|--------------|
| User identifier | `sub` | Ταυτοποιεί τον χρήστη με συνέπεια μεταξύ launches ώστε το ίδιο άτομο να αντιστοιχεί στον ίδιο FastComments SSO χρήστη | Ναι | Ναι, ως μέρος ενός σταθερού εσωτερικού SSO ID |
| Display name | `name` | Εμφάνιση αποδιδόμενη δίπλα στα σχόλια του χρήστη | Ναι (σε περίπτωση απουσίας επιστρέφει ως "Χρήστης LMS") | Ναι |
| Email | `email` | Ταυτοποίηση λογαριασμού, ειδοποιήσεις, moderation, αλληλογραφία υποστήριξης | Προαιρετικό (η ενσωμάτωση λειτουργεί χωρίς αυτό) | Ναι όταν παρέχεται |
| Avatar URL | `picture` | Εμφανίζεται στα σχόλια του χρήστη | Προαιρετικό | Μόνο URL· το FastComments δεν κατεβάζει ούτε φιλοξενεί ξανά την εικόνα |
| Roles | `https://purl.imsglobal.org/spec/lti/claim/roles` | Καθορίζει εάν ο χρήστης είναι διαχειριστής, instructor (moderator) ή learner | Ναι | Παράγονται flags `isAdmin` / `isModerator` στη συνεδρία SSO |
| Course context | `https://purl.imsglobal.org/spec/lti/claim/context` (id, title) | Συνδέει το νήμα σχολίων με το σωστό μάθημα στο LMS | Ναι | Ναι, ως μέρος του επιλυμένου αναγνωριστικού σελίδας |
| Resource link | `https://purl.imsglobal.org/spec/lti/claim/resource_link` (id) | Συνδέει τα σχόλια με τη σωστή δραστηριότητα ή θέση του εργαλείου μέσα στο μάθημα | Ναι όταν υπάρχει | Ναι, ως μέρος του επιλυμένου αναγνωριστικού σελίδας |
| Deployment ID | `https://purl.imsglobal.org/spec/lti/claim/deployment_id` | Κατευθύνει το launch στη σωστή διαμόρφωση tenant του FastComments | Ναι | Ναι, στην εγγραφή διαμόρφωσης LTI του FastComments |

## Claims and Scopes Declared at Registration

Κατά τη Δυναμική Εγγραφή LTI 1.3, το FastComments καταχωρεί τον εαυτό του με `scope: ""` (χωρίς επιπλέον OAuth scopes) και δηλώνει μόνο αυτά τα OpenID Connect claims:

`iss`, `sub`, `name`, `email`, `picture`

Καταχωρεί δύο τύπους μηνυμάτων:

- `LtiResourceLinkRequest` - το πρότυπο launch μαθήματος προς το FastComments.
- `LtiDeepLinkingRequest` - επιτρέπει στους εκπαιδευτές να τοποθετήσουν το εργαλείο FastComments μέσα σε ένα μάθημα.

Δεν ζητούνται επιπλέον access tokens από το LMS.

## LTI Advantage Services Not Requested

| Υπηρεσία / scope | Ζητήθηκε; | Λόγος |
|------------------|----------|-------|
| Names and Role Provisioning Services (NRPS) | Όχι | Η ενσωμάτωση δεν χρειάζεται πλήρη κατάλογο μαθήματος· η ταυτότητα χρήστη φτάνει με κάθε launch |
| Assignment and Grade Services (AGS) - lineitem, score, result scopes | Όχι | Η ενσωμάτωση δεν έχει ενσωμάτωση με το gradebook |
| Deep Linking beyond the standard placement return | Όχι επιπλέον δεδομένα | Το Deep Linking χρησιμοποιείται μόνο για την τοποθέτηση του εργαλείου από τον εκπαιδευτή· δεν παρατίθεται περιεχόμενο μαθήματος |

## Δεδομένα που δεν Συλλέγονται

Beyond LTI itself, FastComments does not request or receive the following from the LMS or user:

| Κατηγορία | Συλλέγεται; |
|----------|------------|
| Student grades | Όχι |
| Assignment submissions | Όχι |
| Attendance records | Όχι |
| Full course rosters | Όχι |
| Government identifiers | Όχι |
| Date of birth | Όχι |
| Postal address or phone number | Όχι |
| Financial information | Όχι |
| LMS administrator credentials | Όχι |

## Όρια Πρόσβασης

- Το FastComments λαμβάνει δεδομένα μόνο μέσα σε ένα εξουσιοδοτημένο LTI 1.3 launch υπογεγραμμένο με τα εγγεγραμμένα κλειδιά του LMS. Η ενσωμάτωση δεν καλεί πίσω το LMS για επιπλέον πληροφορίες.
- Τα launch tokens είναι μίας χρήσης και βραχείας διάρκειας. Επαναλαμβανόμενα ή ληγμένα tokens απορρίπτονται.
- Οι διαχειριστές του LMS ελέγχουν πού αναπτύσσεται το εργαλείο μέσα στην πλατφόρμα τους. Το D2L Brightspace, για παράδειγμα, υποστηρίζει scoping ανά deployment σε οργανωτικές μονάδες και ρυθμίσεις ασφάλειας ανά deployment, που επιτρέπει στους διαχειριστές να περιορίσουν το εργαλείο σε συγκεκριμένα μαθήματα ή οργανικές μονάδες αντί να το κάνουν διαθέσιμο παγκοσμίως. Το Moodle, το Blackboard, το Sakai και το Schoology προσφέρουν αντίστοιχους ελέγχους ανά deployment στις υλοποιήσεις LTI 1.3 τους.

## Αποθήκευση και Διατήρηση

Το FastComments διατηρεί δεδομένα προερχόμενα από LTI για τη διάρκεια της ενεργής υπηρεσίας σχολιασμού και σύμφωνα με τις ρυθμίσεις διατήρησης που ορίζει ο πελάτης. Τα δεδομένα σχολίων αποθηκεύονται σε παραγωγική αποθήκευση κρυπτογραφημένη σε ηρεμία. Σε τερματισμό λογαριασμού ή μετά από γραπτό αίτημα διαγραφής, το FastComments διαγράφει ή ανωνυμοποιεί τα δεδομένα πελατών σύμφωνα με τη σχετική συμφωνία.

Για πλήρεις λεπτομέρειες αποθήκευσης και χειρισμού δεδομένων, δείτε την <a href="https://fastcomments.com/privacy-policy" target="_blank">Πολιτική Απορρήτου FastComments</a>.

## Συχνότητα Αναθεώρησης

Οποιοδήποτε νέο χαρακτηριστικό LTI που θα απαιτούσε πρόσθετα claims, scopes ή υπηρεσίες LTI Advantage αναθεωρείται πριν από την κυκλοφορία για να επιβεβαιωθεί ότι η ζητούμενη πρόσβαση είναι αναγκαία και ανάλογη με το χαρακτηριστικό που παραδίδεται.

## Σύντομη Δήλωση για Ερωτηματολόγια Ασφαλείας

> FastComments εφαρμόζει την αρχή του ελάχιστου προνομίου και την ελαχιστοποίηση δεδομένων στην ενσωμάτωση LTI 1.3. Η ενσωμάτωση χρησιμοποιεί μόνο τα LTI launch claims που απαιτούνται για να αυθεντικοποιήσει τον χρήστη (`sub`, `name`, `email`, `picture`), να προσδιορίσει τον ρόλο του και να αναγνωρίσει το μάθημα και τον πόρο στο οποίο ανήκουν τα σχόλια. Το FastComments δεν ζητά Names and Role Provisioning Services, Assignment and Grade Services, δεδομένα gradebook, παρουσίες, πλήρεις καταλόγους ή πρόσβαση διαχειριστή LMS. Οι διαχειριστές του LMS διατηρούν τον έλεγχο σχετικά με ποιες οργανωτικές μονάδες, μαθήματα και deployments είναι διαθέσιμο το εργαλείο.