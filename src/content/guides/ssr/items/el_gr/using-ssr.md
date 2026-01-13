---
Για να χρησιμοποιήσετε το FastComments SSR, ο πελάτης μπορεί να ανακτήσει HTML από το `https://fastcomments.com/ssr/comments` endpoint.

Αυτό μπορεί να γίνει με διάφορους τρόπους.

### Με το WordPress

Το SSR είναι ενεργοποιημένο από προεπιλογή για χρήστες χωρίς ενεργοποιημένο JS ως εφεδρική λύση στο πρόσθετο του WordPress από την έκδοση `3.10.2`.

### Σε μια ιστοσελίδα

Σε μια ήδη υπάρχουσα εφαρμογή, το SSR μπορεί να προστεθεί με το [παρακάτω παράδειγμα](https://github.com/FastComments/fastcomments-code-examples/tree/master/sso/php/ssr), υποθέτοντας ότι η γλώσσα που χρησιμοποιείται είναι PHP:

[inline-code-attrs-start title = 'Παράδειγμα SSR βασισμένο σε PHP'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
$tenantId = rawurlencode("my-tenant-id");
$urlId = rawurlencode("my-page-url-article-id");
$url = rawurlencode("my-page-url");

$fastcomments_url = "https://fastcomments.com/ssr/comments?tenantId=$tenantId&urlId=$urlId&url=$url";
?>
<iframe
    src="<?php echo $fastcomments_url; ?>"
    horizontalscrolling="no"
    allowtransparency="true"
    frameborder="0"
    title="FastComments"
    width="100%"
    height="1500px"
    style= "width: 1px !important; min-width: 100% !important; border: none !important; overflow: hidden !important;"
></iframe>
[inline-code-end]

Μπορούμε επίσης να δείξουμε το SSR UI μόνο όταν ο χρήστης έχει απενεργοποιημένο το JS:

[inline-code-attrs-start title = 'Παράδειγμα εναλλακτικού SSR με PHP'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
$tenantId = rawurlencode("my-tenant-id");
$urlId = rawurlencode("my-page-url-article-id");
$url = rawurlencode("my-page-url");

$fastcomments_url = "https://fastcomments.com/ssr/comments?tenantId=$tenantId&urlId=$urlId&url=$url";
?>
<noscript>
    <iframe
        src="<?php echo $fastcomments_url; ?>"
        horizontalscrolling="no"
        allowtransparency="true"
        frameborder="0"
        title="FastComments"
        width="100%"
        height="1500px"
        style= "width: 1px !important; min-width: 100% !important; border: none !important; overflow: hidden !important;"
    ></iframe>
</noscript>
[inline-code-end]

Για ένα παράδειγμα που χρησιμοποιεί SSO, [δείτε εδώ](https://github.com/FastComments/fastcomments-code-examples/tree/master/sso/php/ssr).

### Με προ-rendered περιεχόμενο

Το blog μας δημιουργείται κατά τη διάρκεια του build, και παρέχει ένα [καλό παράδειγμα SSR με Handlebars](https://github.com/FastComments/fastcomments-blog/blob/master/src/templates/post.html#L51).

### Οι βασικές παράμετροι

Οι βασικές παράμετροι που πρέπει να περάσετε είναι:
- `tenantId` - Αυτό σας ταυτοποιεί ως πελάτη.
- `urlId` - Αυτό ταυτοποιεί τη σελίδα ή το άρθρο για το οποίο θα φορτωθούν τα σχόλια, και ορίζει πού αποθηκεύονται.
- `url` - Αυτό χρησιμοποιείται για ειδοποιήσεις και σχετικές λειτουργίες για να συνδέεται πίσω στο νήμα σχολίων.

### Προσαρμοσμένο Στυλ

Η SSR έκδοση του widget σχολίων χρησιμοποιεί την ίδια δομή και μηχανή απόδοσης με τη JavaScript έκδοση.

Ως εκ τούτου, όλο το προσαρμοσμένο στυλ που λειτουργεί για το JavaScript commenting widget λειτουργεί και για το SSR. 

### Σημειώσεις

Στο SSR, δεν υπάρχει JavaScript για να ελέγχει το ύψος του αποδοθέντος κοντέινερ. Σε προγράμματα περιήγησης, μπορεί να εμφανιστεί κάθετη γραμμή κύλισης για μεγάλες συζητήσεις.

Ως εκ τούτου, πρέπει να το ρυθμίσετε όπως επιθυμείτε.

---