Τώρα θα δημιουργήσουμε τον προσαρμοσμένο κώδικα FastComments σας. Χρησιμοποιήστε τον οδηγό παρακάτω για να ρυθμίσετε πώς θέλετε να λειτουργεί το FastComments στον ιστότοπό σας GoHighLevel:

[snippet id="gohighlevel-wizard"]

### Διαφορετικοί Τύποι Πλαισίου Σχολίων

Μπορείτε να ρυθμίσετε τη γραμμή `TYPE = 'commenting'` για να αλλάξετε το προϊόν που χρησιμοποιείται (για παράδειγμα μπορείτε να τη αλλάξετε σε `live` για streaming chat ή `collab` για collab chat).

### Τοποθέτηση Του Πλαισίου Σχολίων Όπου Θέλετε

Ας υποθέσουμε ότι θέλετε να τοποθετήσετε πλαίσια σχολίων σε συγκεκριμένα μέρη της σελίδας και όχι στις προεπιλεγμένες θέσεις.
Αλλάξτε αυτή τη γραμμή:

    const TARGET_ELEMENT_ID = ''; // set to use target div mode

Σε:

    const TARGET_ELEMENT_ID = 'fc_box'; // set to use target div mode

Στη συνέχεια, στον επεξεργαστή GHL, κάντε κλικ στο κουμπί "code" και προσθέστε το σημείο όπου θέλετε να εμφανίζονται τα σχόλια:

[inline-code-attrs-start title = 'Div FastComments για GoHighLevel'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<div
  id="fc_box"
  type="commenting"
  urlid="custom-chat-id"
></div>
[inline-code-end]

### Διαφορετικός Τύπος Πλαισίου Σχολίων Ανά Σελίδα

Ας πούμε ότι θέλετε οι χρήστες να επισημαίνουν και να συζητούν κομμάτια κειμένου, ή να χρησιμοποιούν το UI του streaming chat αντί για τα σχόλια.

Πρώτα ακολουθήστε τα βήματα παραπάνω στην ενότητα "Τοποθέτηση Του Πλαισίου Σχολίων Όπου Θέλετε".

Σημειώστε στο μικρό απόσπασμα ότι υπάρχει `type="commenting"`.

Αν θέλετε να ενεργοποιήσετε για παράδειγμα το collab chat αλλάξτε το type σε `type="collab"`.

### Εμφάνιση Μόνο Σε Συγκεκριμένες Σελίδες

Αν δεν ορίσετε δεν ορίσετε `TARGET_ELEMENT_ID`, μπορείτε αντίθετα να ρυθμίσετε τη μεταβλητή `VALID_PATTERNS`, για να καθορίσετε σε ποιες διαδρομές URL θα εμφανίζονται τα σχόλια. Από προεπιλογή, θα εμφανίζονται
σε σελίδες που περιέχουν `/post` στο URL.

### Ρύθμιση Collab Chat

Μπορείτε να πείτε στο collab chat να προσθέτει λειτουργικότητα συνεργασίας μόνο γύρω από HTML μέσα σε μια συγκεκριμένη περιοχή, για παράδειγμα, ας πούμε ότι
προσθέτετε τον κώδικα footer παραπάνω και στη συνέχεια προσθέτετε αυτό το div στο περιεχόμενο της ανάρτησης/σελίδας για να ενεργοποιήσετε το collab chat:

[inline-code-attrs-start title = 'Collab Chat με Καθορισμένο Περιεχόμενο'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<div
  id="fc_box"
  type="collab"
  urlid="custom-chat-id"
><p>This content will have collab chat!</p></div>
[inline-code-end]

Τότε το στοιχείο paragraph μέσα στο `<div>` θα έχει ενεργοποιημένο το collab chat, και τίποτα άλλο στη σελίδα. Αν δεν
βάλετε κανένα περιεχόμενο στο `<div>` τότε θα ενεργοποιήσει το collab chat σε ολόκληρο το σώμα της ανάρτησης.

---