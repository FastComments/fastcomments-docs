Σε αυτό το επόμενο βήμα πρέπει να αντιγράψετε τον έτοιμο κώδικα widget παρακάτω.

Εφόσον είστε συνδεδεμένοι στο FastComments.com, το παρακάτω απόσπασμα κώδικα θα περιέχει ήδη τις πληροφορίες του λογαριασμού σας. Ας το αντιγράψουμε:

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

            // Clean up existing instance
            if (target.fastCommentsInstance) {
                target.fastCommentsInstance.destroy();
            }

            // Clean up existing top bar if it exists
            if (currentTopBar && currentTopBar.parentNode) {
                currentTopBar.parentNode.removeChild(currentTopBar);
            }

            // Create new top bar
            const topBarTarget = document.createElement('div');
            target.parentNode.insertBefore(topBarTarget, target);
            topBarTarget.style.maxWidth = 'var(--layout-max-width)';
            topBarTarget.style.margin = '0 auto';
            currentTopBar = topBarTarget;
            currentWidget = target;

            // Initialize FastComments Collab Chat
            target.fastCommentsInstance = FastCommentsCollabChat(target, {
                tenantId: "demo",
                topBarTarget: topBarTarget
            });

            // Update current pathname
            currentPathname = window.location.pathname;
        }

        // Initial load
        load();

        // Check every 500ms for changes
        setInterval(() => {
            // Reload if pathname changed
            if (window.location.pathname !== currentPathname) {
                console.log('Pathname changed, reloading...');
                load();
                return;
            }

            // Reload if widget was removed
            if (currentWidget && !currentWidget.parentNode) {
                console.log('Widget removed, reloading...');
                load();
                return;
            }

            // Reload if container was emptied
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

Αν δείτε ένα μήνυμα "this is a demo message" μετά την επικόλληση του κώδικα:

- Βεβαιωθείτε ότι είστε συνδεδεμένοι στον λογαριασμό σας στο fastcomments.com.
- Βεβαιωθείτε ότι έχετε ενεργοποιήσει τα cookies τρίτων.
- Στη συνέχεια ανανεώστε αυτή τη σελίδα και αντιγράψτε ξανά το απόσπασμα κώδικα. Θα πρέπει να έχει το `tenantId` συμπληρωμένο με το αναγνωριστικό του tenant σας.