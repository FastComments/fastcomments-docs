Τώρα μπορούμε να αντιγράψουμε το ακόλουθο απόσπασμα κώδικα (χρησιμοποιήστε το κουμπί αντιγραφής επάνω δεξιά στο απόσπασμα):

[inline-code-attrs-start title = 'Κώδικας σχολίων για το Squarespace Blog'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        const tenantId = 'demo'; // το αναγνωριστικό του λογαριασμού σας

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

...then paste in the code area and click save:

<div class="screenshot white-bg">
    <div class="title">Επικόλληση και Αποθήκευση</div>
    <img class="screenshot-image" src="/images/installation-guides/squarespace-step-2-1-add-code-and-save.png" alt="Επικόλληση και Αποθήκευση" />
</div>