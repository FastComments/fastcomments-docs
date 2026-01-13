Στη συνέχεια θα μετακινηθούμε προς τα κάτω στη γραμμή `100`:

<div class="screenshot white-bg">
    <div class="title">Μετακινηθείτε στη γραμμή 100</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-7-line-100.png" alt="Μετακινηθείτε στη γραμμή 100" />
</div>

Τώρα αντιγράψτε το παρακάτω απόσπασμα κώδικα, το οποίο είναι σχεδιασμένο **ειδικά για το Shopify - μην χρησιμοποιείτε αποσπάσματα κώδικα από άλλους οδηγούς**:

[inline-code-attrs-start title = 'Απόσπασμα FastComments για Shopify'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget" class="page-width page-width--narrow"></div>
<script>
    FastCommentsUI(document.getElementById('fastcomments-widget'), {
        tenantId: "demo",
        urlId: window.location.pathname
    });
</script>
[inline-code-end]

Τώρα θέλουμε να τοποθετήσουμε τον δρομέα μας στη `line 101` - αμέσως μετά το `</div>` - και να επικολλήσουμε. Θα πρέπει να έχετε κάτι σαν αυτό:

<div class="screenshot white-bg">
    <div class="title">Προσθέστε τον κώδικα του FastComments</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-8-code-pasted.png" alt="Προσθέστε τον κώδικα του FastComments" />
</div>

Τώρα μπορούμε να αποθηκεύσουμε:

<div class="screenshot white-bg">
    <div class="title">Αποθήκευση</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-9-save.png" alt="Αποθήκευση" />
</div>

---