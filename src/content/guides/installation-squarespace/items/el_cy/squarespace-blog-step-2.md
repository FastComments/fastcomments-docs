Τώρα μπορούμε να αντιγράψουμε το ακόλουθο απόσπασμα κώδικα (χρησιμοποιήστε το κουμπί αντιγραφής πάνω δεξιά στο απόσπασμα):

[inline-code-attrs-start title = 'Κώδικας σχολίων ιστολογίου Squarespace'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        const tenantId = 'demo'; // το αναγνωριστικό του λογαριασμού σας

        function tryLoad() {
            // προσπάθεια φόρτωσης για διαφορετικές διατάξεις
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

...στη συνέχεια επικολλήστε στην περιοχή κώδικα και πατήστε Αποθήκευση:

<div class="screenshot white-bg">
    <div class="title">Επικόλληση και Αποθήκευση</div>
    <img class="screenshot-image" src="/images/installation-guides/squarespace-step-2-1-add-code-and-save.png" alt="Επικόλληση και Αποθήκευση" />
</div>