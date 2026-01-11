---
Για να χρησιμοποιήσετε το FastComments SSR, ο πελάτης μπορεί να λάβει HTML από το `https://fastcomments.com/ssr/comments` endpoint.

Αυτό μπορεί να γίνει με διάφορους τρόπους.

### Με WordPress

Το SSR είναι ενεργοποιημένο από προεπιλογή για χρήστες που έχουν απενεργοποιημένο το JS ως εφεδρική λύση στο WordPress plugin από την έκδοση `3.10.2`.

### Σε μια Ιστοσελίδα

Σε μια ήδη υπάρχουσα εφαρμογή, το SSR μπορεί να προστεθεί με το [παρακάτω παράδειγμα](https://github.com/FastComments/fastcomments-code-examples/tree/master/sso/php/ssr), υποθέτοντας ότι η γλώσσα που χρησιμοποιείται είναι PHP:

[inline-code-attrs-start title = 'Παράδειγμα SSR με PHP'; type = 'php'; isFunctional = false; inline-code-attrs-end]
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

Μπορούμε επίσης να εμφανίζουμε το SSR UI μόνο όταν ο χρήστης έχει απενεργοποιημένο το JS:

[inline-code-attrs-start title = 'Παράδειγμα εφεδρείας SSR με PHP'; type = 'php'; isFunctional = false; inline-code-attrs-end]
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

### Με Προ-Rendered Περιεχόμενο

Το blog μας δημιουργείται κατά το build time, και παρέχει ένα [καλό παράδειγμα SSR με Handlebars](https://github.com/FastComments/fastcomments-blog/blob/master/src/templates/post.html#L51).

### Οι βασικές παράμετροι

Οι βασικές παράμετροι που πρέπει να περάσετε είναι:
- `tenantId` - Αυτό σας ταυτοποιεί ως πελάτη.
- `urlId` - Αυτό ταυτοποιεί τη σελίδα ή το άρθρο για να φορτωθούν τα σχόλια, και ορίζει που αποθηκεύονται.
- `url` - Χρησιμοποιείται για ειδοποιήσεις και σχετικές λειτουργίες ώστε να συνδέεται πίσω στο νήμα σχολίων.

### Προσαρμοσμένο Στυλ

Η έκδοση SSR του comment widget χρησιμοποιεί την ίδια δομή και rendering engine με αυτή του JavaScript.

Ως εκ τούτου, όλο το προσαρμοσμένο στυλ που λειτουργεί για το JavaScript commenting widget λειτουργεί και για το SSR. 

### Σημειώσεις

Με το SSR, δεν υπάρχει JavaScript για να ελέγχει το ύψος του αποδοθέντος container. Στους περιηγητές, μπορεί να εμφανιστεί μία κάθετη γραμμή κύλισης για εκτενείς συζητήσεις.

Ως εκ τούτου, πρέπει να το ρυθμίσετε όπως επιθυμείτε.

---