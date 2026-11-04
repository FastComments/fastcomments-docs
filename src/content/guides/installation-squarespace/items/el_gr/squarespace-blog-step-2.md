Τώρα μπορούμε να αντιγράψουμε το παρακάτω απόσπασμα κώδικα (χρησιμοποιήστε το κουμπί αντιγραφής στην πάνω δεξιά γωνία του αποσπάσματος):

[inline-code-attrs-start title = 'Κώδικας Σχολίων Blog Squarespace'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

...και μετά επικολλήστε στην περιοχή κώδικα και πατήστε αποθήκευση:

<div class="screenshot white-bg">
    <div class="title">Επικόλληση και Αποθήκευση</div>
    <img class="screenshot-image" src="/images/installation-guides/squarespace-step-2-1-add-code-and-save.png" alt="Επικόλληση και Αποθήκευση" />
</div>