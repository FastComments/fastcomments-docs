Στη συνέχεια θα μετακινηθούμε προς τα κάτω στη γραμμή `100`:

<div class="screenshot white-bg">
    <div class="title">Κύλιση στη γραμμή 100</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-7-line-100.png" alt="Κύλιση στη γραμμή 100" />
</div>

Τώρα αντιγράψτε το ακόλουθο απόσπασμα κώδικα, το οποίο έχει σχεδιαστεί **ειδικά για το Shopify - μην χρησιμοποιήσετε αποσπάσματα κώδικα από άλλα σεμινάρια**:

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

Τώρα θέλουμε να τοποθετήσουμε τον δρομέα μας στη `line 101` - right after the `</div>` - and paste. You should have something like this:

<div class="screenshot white-bg">
    <div class="title">Προσθέστε τον κώδικα FastComments</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-8-code-pasted.png" alt="Προσθέστε τον κώδικα FastComments" />
</div>

Τώρα μπορούμε να αποθηκεύσουμε:

<div class="screenshot white-bg">
    <div class="title">Αποθήκευση</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-9-save.png" alt="Αποθήκευση" />
</div>