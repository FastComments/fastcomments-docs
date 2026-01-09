Τώρα ας προσθέσουμε τον κώδικα του widget μας.

Αντιγράψτε τον παρακάτω κώδικα. Βεβαιωθείτε ότι έχετε συνδεθεί στο [fastcomments.com](https://fastcomments.com) και ανανεώστε αυτή τη σελίδα αν όχι, ώστε ο κώδικας να προ-συμπληρωθεί με τις πληροφορίες του λογαριασμού σας, αλλιώς θα εμφανιστεί ο demo κώδικας.

Τώρα ας αντιγράψουμε τον κώδικα:

[inline-code-attrs-start title = 'Κώδικας σχολίων Hostinger'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    FastCommentsUI(document.getElementById('fastcomments-widget'), {
        tenantId: "demo",
        pageTitle: window.parent.document.title,
        urlId: window.parent.location.href,
        url: window.parent.location.href
    });
</script>
[inline-code-end]

Τώρα επιστρέψτε στον δημιουργό του ιστότοπού σας και κάντε κλικ στο `Enter code`:

<div class="screenshot white-bg">
    <div class="title">Εισαγωγή κώδικα</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-2.png" alt="Εισαγωγή κώδικα" />
</div>

### Σημείωση!

Είναι σημαντικό να χρησιμοποιήσετε τον παραπάνω κώδικα και όχι αποσπάσματα κώδικα από άλλη τεκμηρίωση, καθώς αυτό το απόσπασμα έχει σχεδιαστεί ειδικά για το Hostinger.

Τώρα θα πρέπει να έχετε κάτι σαν το παρακάτω, το οποίο εμφανίζεται κενό. Αυτό είναι αναμενόμενο. Μετακινήστε τον δείκτη του ποντικιού πάνω από την περιοχή όπου θα πρέπει να βρίσκεται το widget:

<div class="screenshot white-bg">
    <div class="title">Το widget κώδικα προστέθηκε</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-3.png" alt="Το widget κώδικα προστέθηκε" />
</div>

Τώρα σύρετε το widget στο επιθυμητό μέγεθος, θα δείτε να εμφανίζεται:

<div class="screenshot white-bg">
    <div class="title">Αλλαγή μεγέθους</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-4.png" alt="Αλλαγή μεγέθους" />
</div>

...και τώρα κάντε προεπισκόπηση και αποθήκευση!