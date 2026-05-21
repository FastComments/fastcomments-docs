Αν δεν μπορείτε να εγκαταστήσετε την [Shopify App Store app](https://apps.shopify.com/fastcomments), μπορείτε ακόμα να προσθέσετε το FastComments επεξεργαζόμενοι το theme σας. Αυτός ο τρόπος είναι χρήσιμος όταν θέλετε να συνδέσετε έναν tenant FastComments που ήδη έχετε, ή όταν ενσωματώνετε σε ένα κατάστημα Shopify όπου η εφαρμογή δεν είναι επιλογή.

Η εγκατάσταση μέσω εφαρμογής είναι ο υποστηριζόμενος δρόμος για τα περισσότερα καταστήματα. Επιλέξτε αυτήν μόνο αν η εφαρμογή δεν ταιριάζει.

### Βήμα 1: Απενεργοποιήστε τα εγγενή σχόλια του Shopify

Στο διαχειριστικό του Shopify, πηγαίνετε σε **Blog posts > Manage blogs**, ανοίξτε κάθε blog και θέστε **Comments are disabled** στο δεξί πάνελ. Αποθηκεύστε.

Αυτό σταματά τα ενσωματωμένα σχόλια του Shopify από το να εμφανίζονται παράλληλα με το FastComments.

### Βήμα 2: Ανοίξτε το πρότυπο θέματος ιστολογίου

Στο διαχειριστικό του Shopify:

1. Πηγαίνετε σε **Online Store > Themes**.
2. Υπό το τρέχον θέμα σας, κάντε κλικ **Actions > Edit code**.
3. Στον περιηγητή αρχείων στα αριστερά, ανοίξτε **Sections** και κάντε κλικ στο `main-article.liquid`.

Αυτό είναι το πρότυπο που αποδίδει το Shopify για ένα μεμονωμένο άρθρο ιστολογίου.

### Βήμα 3: Επικολλήστε το απόσπασμα FastComments

Κάντε scroll περίπου στη γραμμή 100 του `main-article.liquid`, αμέσως μετά το κλείσιμο `</div>` του σώματος του άρθρου. Επικολλήστε το ακόλουθο απόσπασμα:

[inline-code-attrs-start title = 'Απόσπασμα Shopify FastComments'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Αντικαταστήστε `"demo"` με το δικό σας Tenant ID από το [fastcomments.com/auth/my-account/api-secret](https://fastcomments.com/auth/my-account/api-secret). Κάντε κλικ στο **Save**.

### Βήμα 4: Εξουσιοδοτήστε το domain του καταστήματός σας

Ανοίξτε ένα άρθρο ιστολογίου στο ζωντανό κατάστημά σας. Εάν δείτε ένα σφάλμα εξουσιοδότησης αντί για το widget σχολίων, το FastComments χρειάζεται να γνωρίζει ότι το κατάστημά σας επιτρέπεται να χρησιμοποιεί αυτόν τον tenant. Δείτε τα [Domain Errors](/guide-installation-shopify.html#shopify-domain-errors).

### Προσθήκη FastComments σε άλλες σελίδες

Το ίδιο απόσπασμα λειτουργεί σε οποιοδήποτε πρότυπο Liquid, συμπεριλαμβανομένων των σελίδων προϊόντων, προσαρμοσμένων σελίδων και της κεντρικής σελίδας. Επικολλήστε το όπου θέλετε να εμφανίζονται τα σχόλια και προσαρμόστε το `urlId` αν θέλετε ένα σταθερό αναγνωριστικό ανά σελίδα (για παράδειγμα, `urlId: "{{ product.id }}"` σε ένα πρότυπο προϊόντος).