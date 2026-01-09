## Προσθήκη widget ζωντανών σχολίων στα άρθρα Notion του Super.so σας

Εκτός από το Collab Chat, μπορείτε να προσθέσετε ένα παραδοσιακό widget σχολίων στο κάτω μέρος των άρθρων Notion σας. Αυτό επιτρέπει στους αναγνώστες να αφήνουν σχόλια και να συζητούν για ολόκληρο το άρθρο.

### Βήματα εγκατάστασης

Αντιγράψτε τον παρακάτω κώδικα και επικολλήστε τον στην ενότητα `Body` των ρυθμίσεων του site σας στο Super.so:

[inline-code-attrs-start title = 'Widget ζωντανών σχολίων FastComments για Super.so'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<script>
    (function () {
        let currentPathname = window.location.pathname;
        let currentWidget = null;

        function load() {
            if (!window.FastCommentsUI) {
                console.log('...no script, trying again...');
                return setTimeout(load, 100);
            }

            const contentArea = document.querySelector('.notion-root');
            if (!contentArea || !contentArea.innerHTML || contentArea.innerHTML.length < 100) {
                console.log('...no content, trying again...');
                return setTimeout(load, 100);
            }

            // Καθαρισμός υπάρχουσας παρουσίας
            if (contentArea.fastCommentsInstance) {
                contentArea.fastCommentsInstance.destroy();
            }

            // Δημιουργία νέου στόχου
            const target = document.createElement('div');
            contentArea.append(target);
            currentWidget = target;

            // Αρχικοποίηση FastComments
            contentArea.fastCommentsInstance = FastCommentsUI(target, {
                tenantId: "demo",
                urlId: window.location.pathname
            });

            // Ενημέρωση του τρέχοντος pathname
            currentPathname = window.location.pathname;
        }

        // Αρχική φόρτωση
        load();

        // Έλεγχος για αλλαγές κάθε 500ms
        setInterval(() => {
            // Επαναφόρτωση εάν το pathname άλλαξε
            if (window.location.pathname !== currentPathname) {
                console.log('Pathname changed, reloading...');
                load();
                return;
            }

            // Επαναφόρτωση εάν το widget αφαιρέθηκε
            if (currentWidget && !currentWidget.parentNode) {
                console.log('Widget removed, reloading...');
                load();
                return;
            }

            // Επαναφόρτωση εάν το container αδειάστηκε
            const contentArea = document.querySelector('.notion-root');
            if (contentArea && contentArea.innerHTML.length < 100) {
                console.log('Container emptied, reloading...');
                load();
            }
        }, 500);
    })();
</script>
[inline-code-end]

### Σημαντικές Σημειώσεις

- Το widget σχολίων θα εμφανίζεται στο κάτω μέρος των άρθρων Notion σας
- Κάθε σελίδα αποκτά το δικό της μοναδικό νήμα σχολίων βάσει της διαδρομής URL
- Βεβαιωθείτε ότι έχετε αντικαταστήσει το `"demo"` με το πραγματικό tenant ID από τον λογαριασμό σας στο FastComments
- Το widget διαχειρίζεται αυτόματα τη δυναμική φόρτωση σελίδων του Super.so

---