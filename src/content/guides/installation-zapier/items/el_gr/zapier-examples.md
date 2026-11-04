## Παραδείγματα Zaps

Μερικές ροές εργασίας που ρυθμίζονται μέσα σε λίγα λεπτά.

**Ενημέρωση για νέα σχόλια.** New Comment, και μετά Slack "Send Channel Message" ή Discord "Send Channel Message". Αντιστοιχίστε το όνομα του σχολιαστή, το κείμενο του σχολίου και το URL της σελίδας στο μήνυμα. Προσθέστε το φίλτρο domain για να ειδοποιείτε διαφορετικό κανάλι ανά ιστότοπο.

**Καταγραφή κάθε σχολίου σε log.** New Comment, και μετά Google Sheets "Create Spreadsheet Row". Προσθέστε το Deleted Comment ως δεύτερο Zap που προσθέτει μια γραμμή με το id του σχολίου, ώστε το φύλλο να λειτουργεί και ως audit trail.

**Email στον συγγραφέα όταν εγκρίνεται ένα σχόλιο.** Updated Comment με φίλτρο Zapier στο Approved is true, και μετά Gmail "Send Email". Επειδή το Updated Comment ενεργοποιείται σε κάθε αλλαγή, το φίλτρο είναι αυτό που κάνει το Zap να αντιδρά μόνο στις εγκρίσεις.

**Προσθήκη σχολιαστών στο CRM ή στη λίστα αλληλογραφίας σας.** New Comment, και μετά HubSpot "Create or Update Contact" ή Mailchimp "Add or Update Subscriber" χρησιμοποιώντας το email του σχολιαστή. Σεβαστείτε την πολιτική απορρήτου σας και την τοπική νομοθεσία πριν προσθέσετε οποιονδήποτε σε λίστα μάρκετινγκ.

**Δημιουργία σχολίου από φόρμα.** Typeform ή Google Forms "New Response", και μετά FastComments Create Comment με το page URL ID που χρησιμοποιεί ο ιστότοπός σας για μαρτυρίες (testimonials). Αφήστε το Approved ανενεργό για να ελέγχετε το καθένα πριν εμφανιστεί.

**Δημοσίευση ανακοινώσεων σε feed.** RSS by Zapier "New Item in Feed", και μετά Create Feed Post με τον τίτλο, το περιεχόμενο και τον σύνδεσμο του στοιχείου.

**Δημιουργία μελών ως χρήστες SSO.** Memberstack, Memberful ή το δικό σας webhook, και μετά Find SSO User ακολουθούμενο από Create SSO User σε λειτουργία "find or create".

**Κλιμάκωση αναφερθέντων σχολίων.** Updated Comment, φιλτραρισμένο σε αριθμό σημάνσεων (flags) πάνω από μηδέν, και μετά Trello "Create Card" ή Linear "Create Issue" με το id του σχολίου και έναν σύνδεσμο προς τη σελίδα διαχείρισης.

**Δημοσίευση σελίδων καθώς βγαίνουν live.** WordPress ή Ghost "New Post", και μετά Create Page με το URL της ανάρτησης, ώστε η σελίδα να είναι καταχωρημένη και περιορισμένη πριν το πρώτο σχόλιο.

**Αρχειοθέτηση διαγραμμένων σχολίων.** Deleted Comment, και μετά Airtable "Create Record" με το πλήρες σχόλιο για συμμόρφωση με απαιτήσεις διατήρησης δεδομένων.