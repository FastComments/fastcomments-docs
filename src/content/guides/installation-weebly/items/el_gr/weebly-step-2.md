Για να λειτουργήσει σωστά η ενσωμάτωση Weebly και FastComments, πρέπει να προσθέσουμε **δύο** μικρά κομμάτια κώδικα.

Το πρώτο απόσπασμα κώδικα είναι για να αποκρύψει το μήνυμα του Weebly «Τα σχόλια είναι κλειστά», και το δεύτερο είναι για να φορτώσει πραγματικά το FastComments.

Πρώτα, αντιγράψτε αυτό το μικρό απόσπασμα κώδικα:

[inline-code-attrs-start title = 'Απόσπασμα Κώδικα Επικεφαλίδας FastComments'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<style>
    #comments {
        display: none;
    }
    #commentArea:not(.loaded) {
        display: none;
    }
    #commentArea.loaded {
        display: block !important;
    }
</style>
[inline-code-end]

Στη συνέχεια, στην ίδια σελίδα ρυθμίσεων από το `Step One`, κάντε κλικ στο `+` δίπλα στο `Post header code`.

<div class="screenshot white-bg">
    <div class="title">Άνοιγμα Κώδικα Επικεφαλίδας Δημοσίευσης</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-1-open-post-header-code.png" alt="Άνοιγμα Κώδικα Επικεφαλίδας Δημοσίευσης" />
</div>

Θα πρέπει να δείτε ένα πεδίο κειμένου να ανοίγει έτσι:

<div class="screenshot white-bg">
    <div class="title">Άνοιγμα Κώδικα Επικεφαλίδας Δημοσίευσης</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-2-post-header-code-open.png" alt="Άνοιγμα Κώδικα Επικεφαλίδας Δημοσίευσης" />
</div>

Τώρα επικολλήστε το απόσπασμα κώδικα:

<div class="screenshot white-bg">
    <div class="title">Απόσπασμα Κώδικα Κεφαλίδας Επικολλημένο</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-3-post-header-code-added.png" alt="Απόσπασμα Κώδικα Κεφαλίδας Επικολλημένο" />
</div>

Στη συνέχεια είναι ο κώδικας υποσέλιδου για να ενεργοποιήσει το FastComments. Κάντε κλικ στο σύμβολο συν δίπλα στο `Post footer code`:

<div class="screenshot white-bg">
    <div class="title">Άνοιγμα Κώδικα Υποσέλιδου Δημοσίευσης</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-4-open-post-footer-code.png" alt="Άνοιγμα Κώδικα Υποσέλιδου Δημοσίευσης" />
</div>

Αντιγράψτε αυτό το απόσπασμα κώδικα που έχει σχεδιαστεί **ειδικά για το Weebly**:

[inline-code-attrs-start title = 'Απόσπασμα Κώδικα Υποσέλιδου FastComments'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        let loaded = false;
        let interval = null;
        function attemptLoad() {
            if (loaded) {
                clearInterval(interval);
                return;
            }
            const comments = document.getElementById('comments');
            if (comments) { // remove show comments button
                comments.remove();
            }
            const commentArea = document.getElementById('commentArea');
            if (!commentArea) {
                return;
            }
            commentArea.innerHTML = '';
            commentArea.classList.add('loaded');
            FastCommentsUI(commentArea, {
                tenantId: "demo",
                urlId: window.location.pathname
            });
            loaded = true;
            clearInterval(interval);
        }
        attemptLoad();
        interval = setInterval(attemptLoad, 300);
    })();
</script>
[inline-code-end]

Τώρα επικολλήστε τον κώδικα υποσέλιδου:

<div class="screenshot white-bg">
    <div class="title">Προστέθηκε Κώδικας Υποσέλιδου Δημοσίευσης</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-5-post-footer-code-added.png" alt="Προστέθηκε Κώδικας Υποσέλιδου Δημοσίευσης" />
</div>

Αυτό ήταν!

---