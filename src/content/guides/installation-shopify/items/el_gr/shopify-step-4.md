Στη συνέχεια θα κάνουμε κύλιση προς τα κάτω στη γραμμή `100`:

<div class="screenshot white-bg">
    <div class="title">Κάντε κύλιση στη γραμμή 100</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-7-line-100.png" alt="Κάντε κύλιση στη γραμμή 100" />
</div>

Τώρα αντιγράψτε το παρακάτω απόσπασμα κώδικα, το οποίο έχει σχεδιαστεί **ειδικά για το Shopify - μην χρησιμοποιήσετε αποσπάσματα κώδικα από άλλα σεμινάρια**:

[inline-code-attrs-start title = 'Απόσπασμα FastComments για Shopify'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget" class="page-width page-width--narrow"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: "demo",
        urlId: window.location.pathname
    }];
</script>
[inline-code-end]

Τώρα θέλουμε να τοποθετήσουμε τον δείκτη μας στη γραμμή `101` - αμέσως μετά το `</div>` - και να επικολλήσουμε. Θα πρέπει να έχετε κάτι σαν το εξής:

<div class="screenshot white-bg">
    <div class="title">Προσθέστε τον κώδικα FastComments</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-8-code-pasted.png" alt="Προσθέστε τον κώδικα FastComments" />
</div>

Τώρα μπορούμε να αποθηκεύσουμε:

<div class="screenshot white-bg">
    <div class="title">Αποθήκευση</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-9-save.png" alt="Αποθήκευση" />
</div>