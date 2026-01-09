## Προσθήκη Ζωντανού Widget Σχολίων σε άρθρα Notion στο Super.so

Εκτός από το Collab Chat, μπορείτε να προσθέσετε ένα παραδοσιακό widget σχολίων στο κάτω μέρος των άρθρων Notion σας. Αυτό επιτρέπει στους αναγνώστες να αφήνουν σχόλια και να έχουν συζητήσεις σχετικά με ολόκληρο το άρθρο.

### Βήματα Εγκατάστασης

Αντιγράψτε τον ακόλουθο κώδικα και επικολλήστε τον στην ενότητα `Body` των ρυθμίσεων του site σας στο Super.so:

[inline-code-attrs-start title = 'Super.so FastComments Ζωντανό Widget Σχολίων'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

            // Ενημέρωση της τρέχουσας διαδρομής
            currentPathname = window.location.pathname;
        }

        // Αρχικό φόρτωμα
        load();

        // Έλεγχος για αλλαγές κάθε 500ms
        setInterval(() => {
            // Φόρτωση ξανά αν άλλαξε η διαδρομή
            if (window.location.pathname !== currentPathname) {
                console.log('Pathname changed, reloading...');
                load();
                return;
            }

            // Φόρτωση ξανά αν το widget αφαιρέθηκε
            if (currentWidget && !currentWidget.parentNode) {
                console.log('Widget removed, reloading...');
                load();
                return;
            }

            // Φόρτωση ξανά αν το περιέκτη αδειάσει
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
- Κάθε σελίδα αποκτά το δικό της μοναδικό νήμα σχολιασμού βάσει της διαδρομής του URL
- Βεβαιωθείτε ότι θα αντικαταστήσετε το `"demo"` με το πραγματικό tenant ID από τον λογαριασμό FastComments σας
- Το widget χειρίζεται αυτόματα το δυναμικό φόρτωμα σελίδων του Super.so

---