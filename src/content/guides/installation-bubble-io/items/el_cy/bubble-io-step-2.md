Κάντε κλικ στο στοιχείο HTML που μόλις προσθέσατε. Στον επεξεργαστή ιδιοτήτων που εμφανίζεται, επικολλήστε τον ακόλουθο κώδικα στο πεδίο HTML:

[inline-code-attrs-start title = 'Απόσπασμα κώδικα Live σχολιασμού Bubble.io'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget" style="width: 100%;height: 100%;"></div>
<script>
    (function fcLoad() {
        function tryLoad() {
            // το Bubble τείνει να αλλάζει το απόσπασμα κώδικα ώστε να είναι ασύγχρονο
            const container = document.getElementById('fastcomments-widget');
            if (!container) {
                return waitRetry();
            }
            if (!window.FastCommentsUI) {
                return waitRetry();
            }
            if (container.fastCommentsSetup) {
                return;
            }
            window.FastCommentsUI(container, {
                tenantId: 'demo',
                pageTitle: top.document.title,
                url: top.location.href,
                urlId: top.location.pathname
            });
            container.fastCommentsSetup = true;
        }
        function waitRetry() {
            setTimeout(tryLoad, 500);
        }
        tryLoad();
    })();
</script>
[inline-code-end]

<div class="screenshot white-bg">
    <div class="title">Εισαγωγή κώδικα FastComments</div>
    <img class="screenshot-image" src="/images/installation-guides/bubble-io-step-2-insert-code.png" alt="Εισαγωγή κώδικα FastComments σε στοιχείο HTML" />
</div>

Σημείωση: Αυτός ο κώδικας περιλαμβάνει μηχανισμό επαναπροσπάθειας για να διασφαλιστεί ότι το FastComments φορτώνει σωστά στο δυναμικό περιβάλλον του Bubble. Άλλα αποσπάσματα κώδικα δεν θα λειτουργήσουν.

Θυμηθείτε να αντικαταστήσετε `'demo'` με το πραγματικό FastComments tenant ID μετά την εγγραφή. Εάν είστε συνδεδεμένοι στο FastComments.com, θα πρέπει να έχει αντικατασταθεί ήδη.