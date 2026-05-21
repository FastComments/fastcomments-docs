Αν δεν μπορείτε να εγκαταστήσετε την [εφαρμογή του Shopify App Store](https://apps.shopify.com/fastcomments), μπορείτε ακόμα να προσθέσετε το FastComments επεξεργαζόμενοι το θέμα σας. Αυτή η μέθοδος είναι χρήσιμη όταν θέλετε να συνδέσετε έναν tenant του FastComments που ήδη κατέχετε, ή όταν ενσωματώνετε σε ένα κατάστημα Shopify όπου η εφαρμογή δεν είναι διαθέσιμη.

Η εγκατάσταση μέσω εφαρμογής είναι η υποστηριζόμενη επιλογή για τα περισσότερα καταστήματα. Επιλέξτε αυτήν μόνο αν η εφαρμογή δεν ταιριάζει.

### Βήμα 1: Απενεργοποιήστε τα ενσωματωμένα σχόλια του Shopify

Στον διαχειριστή του Shopify, μεταβείτε σε **Blog posts > Manage blogs**, ανοίξτε κάθε ιστολόγιο και ρυθμίστε **Comments are disabled** στο δεξιό πάνελ. Αποθηκεύστε.

Αυτό εμποδίζει τα ενσωματωμένα σχόλια του Shopify να εμφανίζονται μαζί με το FastComments.

### Βήμα 2: Ανοίξτε το πρότυπο θέματος του ιστολογίου

Στον διαχειριστή του Shopify:

1. Μεταβείτε στο **Online Store > Themes**.
2. Κάτω από το τρέχον θέμα σας, κάντε κλικ στο **Actions > Edit code**.
3. Στον περιηγητή αρχείων στα αριστερά, ανοίξτε το **Sections** και κάντε κλικ στο `main-article.liquid`.

Αυτό είναι το πρότυπο που αποδίδει το Shopify για ένα μεμονωμένο άρθρο ιστολογίου.

### Βήμα 3: Επικολλήστε το απόσπασμα FastComments

Κάντε κύλιση περίπου στη γραμμή 100 του `main-article.liquid`, αμέσως μετά το κλείσιμο `</div>` του σώματος του άρθρου. Επικολλήστε το ακόλουθο απόσπασμα:

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

Αντικαταστήστε το `"demo"` με το δικό σας Tenant ID από [fastcomments.com/auth/my-account/api-secret](https://fastcomments.com/auth/my-account/api-secret). Κάντε κλικ στο **Save**.

### Βήμα 4: Εξουσιοδοτήστε το domain του καταστήματός σας

Ανοίξτε μια δημοσίευση ιστολογίου στο ζωντανό κατάστημά σας. Εάν δείτε ένα σφάλμα εξουσιοδότησης αντί για το widget σχολίων, το FastComments πρέπει να γνωρίζει ότι το κατάστημά σας επιτρέπεται να χρησιμοποιεί αυτόν τον tenant. Δείτε [Σφάλματα domain](/guide-installation-shopify.html#shopify-domain-errors).

### Προσθήκη FastComments σε άλλες σελίδες

Το ίδιο απόσπασμα λειτουργεί σε οποιοδήποτε πρότυπο Liquid, συμπεριλαμβανομένων των σελίδων προϊόντων, προσαρμοσμένων σελίδων και της αρχικής σελίδας. Επικολλήστε το εκεί όπου θέλετε να εμφανίζονται τα σχόλια και προσαρμόστε το `urlId` αν θέλετε έναν σταθερό αναγνωριστικό ανά σελίδα (για παράδειγμα, `urlId: "{{ product.id }}"` σε ένα πρότυπο προϊόντος).