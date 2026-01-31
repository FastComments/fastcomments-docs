Τώρα θα αντιγράψουμε τον κώδικά μας. Εάν το απόσπασμα κώδικα λέει `tenantId: "demo"` στη γραμμή 6 τότε πρέπει να συνδεθείτε στον λογαριασμό σας στο FastComments και στη συνέχεια να ανανεώσετε αυτήν τη σελίδα ώστε το αντιγραμμένο απόσπασμα να έχει το id του λογαριασμού σας.

[inline-code-attrs-start title = 'Απόσπασμα Systeme.io'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: "demo"
    }];
</script>
[inline-code-end]

Τώρα επικολλήστε το στον επεξεργαστή και κάντε κλικ στο κουμπί Αποθήκευση:

<div class="screenshot white-bg">
    <div class="title">Προσθέστε τον κώδικα του FastComments</div>
    <img class="screenshot-image" src="/images/installation-guides/systeme-add-code.png" alt="Προσθέστε τον κώδικα του FastComments" />
</div>

... στη συνέχεια αποθηκεύστε τον ιστότοπό σας. Αυτό ήταν!

---