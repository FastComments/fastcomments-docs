Τώρα ας προσθέσουμε τον κώδικα του widget μας.

Αντιγράψτε τον παρακάτω κώδικα. Βεβαιωθείτε ότι έχετε συνδεθεί στο [fastcomments.com](https://fastcomments.com) 
και ανανεώστε αυτή τη σελίδα αν δεν είστε, ώστε ο κώδικας να γεμίσει αυτόματα με τις πληροφορίες του λογαριασμού σας, αλλιώς θα εμφανίσει τον δοκιμαστικό κώδικα.

Τώρα ας αντιγράψουμε τον κώδικα:

[inline-code-attrs-start title = 'Κώδικας σχολίων Hostinger'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: "demo",
        pageTitle: window.parent.document.title,
        urlId: window.parent.location.href,
        url: window.parent.location.href
    }];
</script>
[inline-code-end]

Τώρα επιστρέψτε στον δημιουργό του ιστότοπού σας και κάντε κλικ στο `Enter code`:

<div class="screenshot white-bg">
    <div class="title">Εισαγωγή κώδικα</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-2.png" alt="Εισαγωγή κώδικα" />
</div>

### Σημείωση!

Είναι σημαντικό να χρησιμοποιήσετε τον παραπάνω κώδικα και όχι τα αποσπάσματα κώδικα από άλλη τεκμηρίωση, καθώς αυτό το απόσπασμα έχει διαμορφωθεί ειδικά για το Hostinger.

Τώρα θα πρέπει να δείτε κάτι σαν το παρακάτω, το οποίο φαίνεται κενό. Αυτό είναι αναμενόμενο. Μετακινήστε τον δείκτη του ποντικιού πάνω στην περιοχή όπου θα πρέπει να εμφανίζεται το widget:

<div class="screenshot white-bg">
    <div class="title">Προστέθηκε το widget</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-3.png" alt="Προστέθηκε το widget" />
</div>

Τώρα σύρετε το widget στο επιθυμητό μέγεθος — θα το δείτε να εμφανίζεται:

<div class="screenshot white-bg">
    <div class="title">Αλλάξτε το μέγεθος</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-4.png" alt="Αλλάξτε το μέγεθος" />
</div>

...και τώρα κάντε προεπισκόπηση και αποθήκευση!