[related-parameter-start name = 'enableViewCounts'; type = 'boolean'; related-parameter-end]

Προεπιλεγμένα, το FastComments δεν παρακολουθεί ποιος είδε κάθε σχόλιο ούτε παρέχει στατιστικά σχετικά με αυτό.

Ωστόσο, μπορούμε να ενεργοποιήσουμε αυτή τη δυνατότητα, και τότε το σύστημα θα αρχίσει να παρακολουθεί όταν κάθε χρήστης κάνει κύλιση σε ένα σχόλιο.

Όταν συμβεί αυτό, ένας μετρητής δίπλα σε ένα εικονίδιο ματιού που εμφανίζεται σε κάθε σχόλιο θα αυξάνεται. Ο μετρητής ενημερώνεται σε πραγματικό χρόνο και συντομεύεται σύμφωνα με την τοπική ρύθμιση (locale) του χρήστη.

Μπορούμε να το ενεργοποιήσουμε ρυθμίζοντας τη σημαία **enableViewCounts** σε true:

[code-example-start config = {enableViewCounts: true}; linesToHighlight = [6]; title = 'Enabling Comment View Counts'; code-example-end]

Αυτό μπορεί να προσαρμοστεί χωρίς κώδικα, στη σελίδα προσαρμογής του widget:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-view-counts']; selector = '.enable-view-counts'; title='Enabling Comment View Counts' app-screenshot-end]

Παρακολουθούμε το user id* που είδε το σχόλιο, έτσι ώστε αν δείτε ξανά το σχόλιο να μην αυξάνεται. Αν δείτε ξανά το σχόλιο μετά από δύο χρόνια, ο μετρητής θα αυξηθεί ξανά.

- *Σημείωση: ή το anon session id, ή η IP του χρήστη ως τιμή κατακερματισμένη.

---