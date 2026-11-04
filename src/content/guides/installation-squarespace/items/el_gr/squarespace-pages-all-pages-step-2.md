Τώρα μπορούμε να αντιγράψουμε το παρακάτω απόσπασμα κώδικα. Χρησιμοποιήστε το κουμπί αντιγραφής που εμφανίζεται στην πάνω δεξιά γωνία του αποσπάσματος.

Υπάρχουν μερικά πράγματα που μπορείτε να ρυθμίσετε στον κώδικα, δείτε τις γραμμές 4 έως 7.

[inline-code-attrs-start title = 'Κώδικας σχολίων για όλες τις σελίδες Squarespace'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        const tenantId = 'demo'; // your account id

        function tryLoad() {
            // try to load for different layouts
            let targetDiv = document.querySelector('.blog-item-comments-content');
            if (!targetDiv) {
                targetDiv = document.getElementById('fastcomments-widget');
            }
            window.FastCommentsUI(targetDiv, {
                tenantId
            });
        }

        tryLoad();
    })();
</script>

[inline-code-end]

...στη συνέχεια επικολλήστε στην περιοχή κώδικα και κάντε κλικ στην αποθήκευση. Θα πρέπει να μοιάζει έτσι, με τον κώδικα στο μπλοκ `FOOTER`:

<div class="screenshot white-bg">
    <div class="title">Επικόλληση και αποθήκευση</div>
    <img class="screenshot-image" src="/images/installation-guides/squarespace-pages-all-pages-step-2-1-add-code-and-save.png" alt="Επικόλληση και αποθήκευση" />
</div>

Αν αντιμετωπίζετε προβλήματα, βεβαιωθείτε ότι κοντά στο κάτω μέρος δεν γράφει `"tenantId": "demo"`. Θα πρέπει να εμφανίζεται το tenant id σας αν είστε συνδεδεμένοι. Αν όχι, επικοινωνήστε με την υποστήριξη.