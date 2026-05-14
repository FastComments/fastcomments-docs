#### "Το token εγγραφής δεν βρέθηκε, έληξε ή έχει ήδη χρησιμοποιηθεί"

Το token στη διεύθυνση εγγραφής σας (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">λάβετε το εδώ</a>) είναι έγκυρο για 30 λεπτά και μπορεί να χρησιμοποιηθεί μόνο μία φορά. Αν το LMS σας χρειάστηκε περισσότερο από αυτόν τον χρόνο, ή αν η εγγραφή επαναλήφθηκε μετά από επιτυχία, το token θα απορριφθεί. Δημιουργήστε ένα νέο URL στη σελίδα ρυθμίσεων FastComments LTI 1.3 Configuration και ξεκινήστε ξανά.

#### "Platform rejected registration"

Το LMS σας αρνήθηκε τη χειραψία εγγραφής. Οι πιο συνηθισμένες αιτίες:

- **Tool already registered with the same client name.** Ορισμένες πλατφόρμες (ιδιαίτερα η D2L) απορρίπτουν μια δεύτερη εγγραφή του "FastComments" έως ότου διαγραφεί η προηγούμενη. Αφαιρέστε το παλιό εργαλείο στο LMS σας και δοκιμάστε ξανά.
- **Wrong field in the LMS.** Βεβαιωθείτε ότι επικολλήσατε το URL στο πεδίο **registration / tool initiation registration endpoint**, όχι στο πεδίο launch URL ή login URL.
- **The LMS doesn't actually support Dynamic Registration.** Παλαιότερες εκδόσεις του Moodle και του Blackboard διαφημίζουν LTI 1.3 αλλά επιτρέπουν μόνο χειροκίνητη ρύθμιση. Ελέγξτε τα έγγραφα της πλατφόρμας σας.

#### "Failed to fetch platform configuration"

Το FastComments δεν μπόρεσε να διαβάσει το έγγραφο openid-configuration του LMS σας. Αυτό είναι σπάνιο και συνήθως σημαίνει ότι το LMS παρείχε ένα κακώς μορφοποιημένο ή μη προσβάσιμο discovery URL. Επικοινωνήστε με την υποστήριξη του LMS σας.

#### Εκκίνηση εμφανίζει "Configuration not found"

Είτε η διαμόρφωση στο FastComments διαγράφηκε, είτε η εκκίνηση προήλθε από ένα ζευγάρι `iss`/`client_id` που δεν αναγνωρίζουμε. Αν τη διαγράψατε και επανεγγραφήκατε, ζητήστε από το LMS σας να αφαιρέσει και να επαναπροσθέσει το εργαλείο FastComments ώστε να λάβει το νέο client_id.

#### Εκκίνηση εμφανίζει "Deployment not registered"

Ξεκινήσατε το FastComments από μια ανάπτυξη Brightspace/Moodle/Blackboard διαφορετική από αυτή στην οποία ξεκίνησε αρχικά. Το FastComments "καρφώνει" το `deployment_id` στην πρώτη εκκίνηση ως έλεγχο ασφαλείας. Για να προσθέσετε μια νέα ανάπτυξη κάτω από τον ίδιο client, επικοινωνήστε με την υποστήριξη — θα προσθέσουμε το deployment ID στη διαμόρφωση.

#### Εκκίνηση εμφανίζει "Unsupported message_type"

Το LMS απέστειλε ένα μήνυμα LTI που το FastComments δεν χειρίζεται (π.χ. `LtiSubmissionReviewRequest`). Το FastComments υποστηρίζει μόνο τις τυπικές ροές resource-link launch και deep-linking. Επικοινωνήστε μαζί μας αν χρειάζεστε να προστεθεί ένας συγκεκριμένος τύπος μηνύματος.

#### Το iframe δεν αλλάζει μέγεθος

Τα περισσότερα LMS αλλάζουν αυτόματα το μέγεθος των LTI iframes. Αν το δικό σας δεν το κάνει, ελέγξτε ότι οι ρυθμίσεις εκκίνησης του LMS επιτρέπουν στο εργαλείο να στέλνει postMessage συμβάντα στο γονικό πλαίσιο. Το FastComments εκπέμπει τόσο μηνύματα αλλαγής μεγέθους σε Canvas-style (`lti.frameResize`) όσο και σύμφωνα με το IMS-spec (`org.imsglobal.lti.frameResize`).

---