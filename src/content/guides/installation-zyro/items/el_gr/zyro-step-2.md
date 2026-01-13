Τώρα ας προσθέσουμε τον κώδικα του widget μας.

Αντιγράψτε τον παρακάτω κώδικα. Βεβαιωθείτε ότι έχετε συνδεθεί στο [fastcomments.com](https://fastcomments.com) 
και φορτώστε ξανά αυτή τη σελίδα αν όχι, ώστε ο κώδικας να προ-συμπληρωθεί με τις πληροφορίες του λογαριασμού σας, αλλιώς θα εμφανιστεί ο demo κώδικας.

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

Τώρα επιστρέψτε στον δημιουργό της ιστοσελίδας μας και κάντε κλικ στο `Enter code`:

<div class="screenshot white-bg">
    <div class="title">Εισαγωγή κώδικα</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-2.png" alt="Εισαγωγή κώδικα" />
</div>

### Σημείωση!

Είναι σημαντικό να χρησιμοποιήσετε τον παραπάνω κώδικα και όχι τα αποσπάσματα κώδικα από άλλη τεκμηρίωση, καθώς αυτό το απόσπασμα έχει σχεδιαστεί ειδικά
για το Zyro.

Τώρα θα πρέπει να έχετε κάτι σαν το παρακάτω, το οποίο εμφανίζεται κενό. Αυτό είναι αναμενόμενο. Μετακινήστε το δείκτη του ποντικιού πάνω στην περιοχή
όπου θα πρέπει να είναι το widget:

<div class="screenshot white-bg">
    <div class="title">Το widget προστέθηκε</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-3.png" alt="Το widget προστέθηκε" />
</div>

Τώρα σύρετε το widget στο επιθυμητό μέγεθος, θα το δείτε να εμφανίζεται:

<div class="screenshot white-bg">
    <div class="title">Αλλάξτε το μέγεθος</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-4.png" alt="Αλλάξτε το μέγεθος" />
</div>

...και τώρα κάντε προεπισκόπηση και αποθήκευση!