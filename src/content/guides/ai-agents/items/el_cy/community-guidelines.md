Το πεδίο **Community guidelines** στη φόρμα επεξεργασίας είναι ένα προαιρετικό μπλοκ κειμένου πολιτικής που περιλαμβάνεται στο μήνυμα περιεχομένου ρόλου χρήστη σε κάθε εκτέλεση για αυτόν τον agent. Είναι φραγμένο ως μη αξιόπιστο κείμενο (το ίδιο φράγμα που εφαρμόζει η πλατφόρμα σε σώματα σχολίων και άλλο περιεχόμενο που παρέχεται από χρήστες), επομένως το μοντέλο το αντιμετωπίζει ως αναφορά πολιτικής, όχι ως οδηγία συστήματος. Είναι το κανόνιστικό σημείο για να καταγράψετε "τι επιτρέπεται και τι δεν επιτρέπεται σε αυτόν τον ιστότοπο", ώστε ο agent να το εφαρμόζει με συνέπεια.

### Πώς διαφέρει από το αρχικό prompt

- **Initial prompt** - ο ρόλος του agent και το στυλ λήψης αποφάσεων. "You are a moderator. Prefer warning over banning."
- **Community guidelines** - οι κανόνες της κοινότητάς σας, σε γλώσσα πολιτικής. "No personal attacks. No promotional links from accounts under 24 hours old. Off-topic comments may be removed if a thread is heated."

Και τα δύο ρέουν στο ίδιο παράθυρο συμφραζομένων, αλλά εισέρχονται σε διαφορετικά στρώματα - το αρχικό prompt είναι μέρος του ρόλου συστήματος, το έγγραφο οδηγιών είναι φραγμένο κείμενο μέσα στο μήνυμα περιεχομένου ρόλου χρήστη. Το διαχωρισμό τον κάνει πιο εύκολο να επεξεργαστείτε όταν θέλετε να ενημερώσετε το ένα χωρίς να ξαναδιαβάσετε το άλλο.

### Τι είναι ένα καλό έγγραφο οδηγιών

Ένα σύντομο, συγκεκριμένο έγγραφο γραμμένο από άνθρωπο. Οι λίστες λειτουργούν καλύτερα από την πρόζα:

[inline-code-attrs-start title = 'Παράδειγμα Οδηγιών Κοινότητας'; type='text' inline-code-attrs-end]
[inline-code-start]
Allowed:
- Substantive disagreement, even strongly worded.
- Links to original sources, even from new accounts.
- Off-topic asides if the parent thread permits them.

Not allowed:
- Personal attacks against specific named users.
- Doxxing or sharing of private information.
- Coordinated promotional activity (multiple comments pushing the same external link).
- Comments that exist only to derail discussion.

Borderline:
- Strong language without a target. Allowed if not directed at a person.
- Political topics outside the page subject. Off-topic; warn first, don't remove unless persistent.
[inline-code-end]

Ο agent εφαρμόζει αυτό σε κάθε εκτέλεση. Εάν αλλάξετε τις οδηγίες, η αλλαγή τίθεται σε ισχύ στο επόμενο trigger - οι προηγούμενες εκτελέσεις δεν επανεκτιμώνται αναδρομικά.

### Τι να μην βάλετε εδώ

- **Output formatting instructions** ("reply in HTML", "use emoji"). Αυτά ανήκουν στο [initial prompt](#personality-prompt).
- **Localized text.** Το έγγραφο οδηγιών, όπως και το prompt, είναι **English-only** για τον ίδιο λόγο - η μηχανική μετάφραση μπορεί να αλλάξει τη συμπεριφορά του agent αθόρυβα. Εάν έχετε πολιτικές που διαφέρουν ανά τοποθεσία, γράψτε τις όλες στα Αγγλικά σε αυτό το έγγραφο και δομήστε το έγγραφο ως "for German-language pages: ..."
- **Long quotations of external policies.** Παραφράστε. Το μεγάλο πλαίσιο κοστίζει tokens σε κάθε εκτέλεση.
- **PII or secrets.** Αυτό το κείμενο στέλνεται στον πάροχο LLM σε κάθε εκτέλεση.

### Μήκος

Το πεδίο περιορίζεται σε **4000 χαρακτήρες** (επιβάλλεται τόσο από τη φόρμα όσο και από τη διαδρομή αποθήκευσης). Το κόστος tokens σε κάθε εκτέλεση είναι ανάλογο με το μήκος, οπότε ακόμα και εντός του ορίου μερικές εκατοντάδες λέξεις είναι συνήθως αρκετές. Αν οι πολιτικές της κοινότητάς σας εκτείνονται σε πολλές σελίδες, συνοψίστε τα τμήματα που χρειάζεται ο agent σε μία περίληψη ειδικά για αυτό το πεδίο.

### Διαχείριση εκδόσεων

Δεν υπάρχει ενσωματωμένο ιστορικό εκδόσεων για το έγγραφο οδηγιών - η τελευταία αποθηκευμένη τιμή είναι αυτή που χρησιμοποιεί ο agent. Εάν θέλετε ιστορικό, αντιγράψτε το έγγραφο στο δικό σας σύστημα καταγραφής πριν από κάθε σημαντική επεξεργασία. Η ροή [Refine Prompts](#refining-prompts) μπορεί να καταγράψει αλλαγές στην *αρχική προτροπή* αλλά δεν διατηρεί εκδόσεις για το έγγραφο οδηγιών.

---