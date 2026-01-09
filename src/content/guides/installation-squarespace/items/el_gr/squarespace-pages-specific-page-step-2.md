Τώρα μπορούμε να αντιγράψουμε το παρακάτω απόσπασμα κώδικα. Χρησιμοποιήστε το κουμπί αντιγραφής που εμφανίζεται επάνω δεξιά στο απόσπασμα.

Υπάρχουν μερικά στοιχεία που μπορείτε να ρυθμίσετε στον κώδικα — δείτε τις γραμμές 4 έως 7.

[inline-code-attrs-start title = 'Κώδικας για μεμονωμένη σελίδα Squarespace'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        const tenantId = 'demo'; // το tenantId του λογαριασμού σας

        function tryLoad() {
            window.FastCommentsUI(document.getElementById('fastcomments-widget'), {
                tenantId
            });
        }

        tryLoad();
    })();
</script>

[inline-code-end]

Θα πρέπει να μοιάζει με αυτό:

<div class="screenshot white-bg">
    <div class="title">Επικόλληση και Αποθήκευση</div>
    <img class="screenshot-image" src="/images/installation-guides/squarespace-pages-specific-page-step-2-1-add-code-and-save.png" alt="Επικόλληση και Αποθήκευση" />
</div>

Τώρα κάντε κλικ στο κουμπί Αποθήκευση επάνω δεξιά.

Σημειώστε ότι η επιλογή `Preview in Safe Mode` δεν θα λειτουργήσει, αλλά το widget θα εμφανιστεί όταν επισκεφτείτε τον ιστότοπό σας.

Εάν αντιμετωπίζετε προβλήματα, βεβαιωθείτε ότι κοντά στο κάτω μέρος δεν αναγράφεται `"tenantId": "demo"`. Θα πρέπει να εμφανίζει το tenant id σας εάν είστε συνδεδεμένοι. Εάν όχι, επικοινωνήστε με την υποστήριξη.