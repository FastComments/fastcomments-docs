---
Στο επόμενο βήμα πρέπει να αντιγράψετε τον προκατασκευασμένο κώδικα widget παρακάτω.

Όσο είστε συνδεδεμένοι στο FastComments.com, το παρακάτω απόσπασμα κώδικα θα περιέχει ήδη τις πληροφορίες του λογαριασμού σας. Ας το αντιγράψουμε:

[inline-code-attrs-start title = 'Κώδικας Super.so FastComments Collab Chat'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

            // Καθαρισμός υπάρχουσας περίπτωσης
            if (target.fastCommentsInstance) {
                target.fastCommentsInstance.destroy();
            }

            // Καθαρισμός υπάρχουσας πάνω γραμμής αν υπάρχει
            if (currentTopBar && currentTopBar.parentNode) {
                currentTopBar.parentNode.removeChild(currentTopBar);
            }

            // Δημιουργία νέας πάνω γραμμής
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

        // Αρχικό φόρτωμα
        load();

        // Έλεγχος για αλλαγές κάθε 500ms
        setInterval(() => {
            // Επαναφόρτωση αν το pathname άλλαξε
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

            // Επαναφόρτωση αν το container αδειάστηκε
            const target = document.querySelector('.super-content');
            if (target && target.innerHTML.length < 100) {
                console.log('Container emptied, reloading...');
                load();
            }
        }, 500);
    })();
</script>
[inline-code-end]

Now paste in the `Body` area:

<div class="screenshot white-bg">
    <div class="title">Επικολλημένος κώδικας</div>
    <img class="screenshot-image" src="/images/installation-guides/super-so-step-3-pasted-code.png" alt="Επικολλημένος κώδικας" />
</div>

If you see a "this is a demo message" after pasting the code:

- Βεβαιωθείτε ότι είστε συνδεδεμένοι στο λογαριασμό σας στο fastcomments.com.
- Βεβαιωθείτε ότι έχετε ενεργοποιημένα τα cookies τρίτων.
- Έπειτα ανανεώστε αυτή τη σελίδα και αντιγράψτε ξανά το απόσπασμα κώδικα. Θα πρέπει να έχει το `tenantId` συμπληρωμένο με το αναγνωριστικό του tenant σας.

---