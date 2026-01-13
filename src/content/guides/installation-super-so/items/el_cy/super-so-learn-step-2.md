---
Στο επόμενο βήμα πρέπει να αντιγράψετε τον έτοιμο κώδικα του widget παρακάτω.

Εφόσον είστε συνδεδεμένος στο FastComments.com το παρακάτω απόσπασμα κώδικα θα περιέχει ήδη τις πληροφορίες του λογαριασμού σας. Ας το αντιγράψουμε:

[inline-code-attrs-start title = 'Κώδικας Συνεργατικού Chat FastComments για Super.so'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>
<script>
    (function () {
        let currentPathname = window.location.pathname;
        let currentWidget = null;
        let currentTopBar = null;

        function load() {
            if (!window.FastCommentsCollabChat) {
                console.log('...no script, trying again...');
                return setTimeout(load, 100);
            }

            const target = document.querySelector('.super-content');
            if (!target || !target.innerHTML || target.innerHTML.length < 100) {
                console.log('...no content, trying again...');
                return setTimeout(load, 100);
            }

            // Καθαρισμός υπάρχουσας παρουσίας
            if (target.fastCommentsInstance) {
                target.fastCommentsInstance.destroy();
            }

            // Καθαρισμός υπάρχουσας κορυφαίας γραμμής αν υπάρχει
            if (currentTopBar && currentTopBar.parentNode) {
                currentTopBar.parentNode.removeChild(currentTopBar);
            }

            // Δημιουργία νέας κορυφαίας γραμμής
            const topBarTarget = document.createElement('div');
            target.parentNode.insertBefore(topBarTarget, target);
            topBarTarget.style.maxWidth = 'var(--layout-max-width)';
            topBarTarget.style.margin = '0 auto';
            currentTopBar = topBarTarget;
            currentWidget = target;

            // Αρχικοποίηση FastComments Collab Chat
            target.fastCommentsInstance = FastCommentsCollabChat(target, {
                tenantId: "demo",
                topBarTarget: topBarTarget
            });

            // Ενημέρωση τρέχοντος pathname
            currentPathname = window.location.pathname;
        }

        // Αρχική φόρτωση
        load();

        // Έλεγχος για αλλαγές κάθε 500ms
        setInterval(() => {
            // Επαναφόρτωση αν αλλάξει το pathname
            if (window.location.pathname !== currentPathname) {
                console.log('Pathname changed, reloading...');
                load();
                return;
            }

            // Επαναφόρτωση αν το widget αφαιρέθηκε
            if (currentWidget && !currentWidget.parentNode) {
                console.log('Widget removed, reloading...');
                load();
                return;
            }

            // Επαναφόρτωση αν ο container αδειάσει
            const target = document.querySelector('.super-content');
            if (target && target.innerHTML.length < 100) {
                console.log('Container emptied, reloading...');
                load();
            }
        }, 500);
    })();
</script>
[inline-code-end]

Τώρα επικολλήστε στην περιοχή `Body`:

<div class="screenshot white-bg">
    <div class="title">Επικολλημένος Κώδικας</div>
    <img class="screenshot-image" src="/images/installation-guides/super-so-step-3-pasted-code.png" alt="Επικολλημένος Κώδικας" />
</div>

If you see a "this is a demo message" after pasting the code:

- Βεβαιωθείτε ότι έχετε συνδεθεί στον λογαριασμό σας στο fastcomments.com.
- Ενεργοποιήστε τα cookies τρίτων (3rd party cookies).
- Στη συνέχεια ανανεώστε αυτή τη σελίδα και αντιγράψτε ξανά το απόσπασμα κώδικα. Πρέπει να έχει `tenantId` συμπληρωμένο με το αναγνωριστικό του tenant σας.

---