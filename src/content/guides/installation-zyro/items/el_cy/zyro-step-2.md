Τώρα ας προσθέσουμε τον κώδικα του widget μας.

Αντιγράψτε τον παρακάτω κώδικα. Θα θέλετε να βεβαιωθείτε ότι έχετε συνδεθεί στο [fastcomments.com](https://fastcomments.com) 
και να επαναφορτώσετε αυτή τη σελίδα αν δεν είστε, ώστε ο κώδικας να προ-συμπληρωθεί με τις πληροφορίες του λογαριασμού σας, αλλιώς θα εμφανίσει τον demo κώδικα.

Τώρα ας αντιγράψουμε τον κώδικα:

[inline-code-attrs-start title = 'Κώδικας σχολίων Zyro'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Τώρα ας επιστρέψουμε στον κατασκευαστή του ιστότοπού μας και κάντε κλικ στο `Enter code`:

<div class="screenshot white-bg">
    <div class="title">Εισαγωγή κώδικα</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-2.png" alt="Εισαγωγή Κώδικα" />
</div>

### Σημείωση!

Είναι σημαντικό να χρησιμοποιήσετε τον παραπάνω κώδικα και όχι τα αποσπάσματα κώδικα από άλλη τεκμηρίωση, καθώς αυτό το απόσπασμα έχει δημιουργηθεί ειδικά
για το Zyro.

Τώρα θα πρέπει να έχετε κάτι σαν το παρακάτω, το οποίο φαίνεται κενό. Αυτό είναι αναμενόμενο. Μετακινήστε το ποντίκι πάνω στην περιοχή
όπου θα πρέπει να εμφανίζεται το widget:

<div class="screenshot white-bg">
    <div class="title">Το widget κώδικα προστέθηκε</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-3.png" alt="Το widget κώδικα προστέθηκε" />
</div>

Τώρα σύρετε το widget στο επιθυμητό μέγεθος — θα το δείτε να εμφανίζεται:

<div class="screenshot white-bg">
    <div class="title">Αλλάξτε το μέγεθος</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-4.png" alt="Αλλάξτε το μέγεθος" />
</div>

...και τώρα κάντε προεπισκόπηση και αποθήκευση!