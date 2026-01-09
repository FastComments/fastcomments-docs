Τώρα μπορούμε να αντιγράψουμε το ακόλουθο απόσπασμα κώδικα. Χρησιμοποιήστε το κουμπί αντιγραφής που εμφανίζεται πάνω δεξιά στο απόσπασμα.

Υπάρχουν μερικά στοιχεία που μπορείτε να ρυθμίσετε στον κώδικα, δείτε τις γραμμές 4 έως 7.

[inline-code-attrs-start title = 'Κώδικας σχολίων για όλες τις σελίδες Squarespace'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        const tenantId = 'demo'; // το id του λογαριασμού σας

        function tryLoad() {
            // προσπαθεί να φορτώσει για διαφορετικές διατάξεις
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

...then paste in the code area and click save. It should look like this, with the code in the `FOOTER` block:

<div class="screenshot white-bg">
    <div class="title">Επικόλληση και Αποθήκευση</div>
    <img class="screenshot-image" src="/images/installation-guides/squarespace-pages-all-pages-step-2-1-add-code-and-save.png" alt="Επικόλληση και Αποθήκευση" />
</div>

If you're having issues, make sure near the bottom it doesn't say `"tenantId": "demo"`. It should show your tenant id if you are logged in. If not, reach out to support.