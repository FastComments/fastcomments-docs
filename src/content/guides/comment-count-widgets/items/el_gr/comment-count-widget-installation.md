Το Comment Count Widget ειναι σχεδιασμενο για την εμφανιση του αριθμου σχολιων μιας μεμονωμενης σελιδας. Ειναι ελαφρυ και παρεχει ενημερωσεις σε πραγματικο χρονο αν ρυθμιστει.

## Βασικη Εγκατασταση

[inline-code-attrs-start title = 'Comment Count Widget Installation'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/widget-comment-count.min.js"></script>
<div id="comment-count"></div>
<script>
    window.FastCommentsCommentCount(document.getElementById('comment-count'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

## Επιλογες Διαμορφωσης

Η συναρτηση `FastCommentsCommentCount` δεχεται τις ακολουθες επιλογες διαμορφωσης:

- **tenantId** (απαιτειται): Το FastComments tenant ID σας
- **urlId** (προαιρετικο): Ο αναγνωριστης σελιδας. Προεπιλογη ειναι `window.location.href` αν δεν οριστει
- **numberOnly** (προαιρετικο): Αν `true`, εμφανιζει μονο τον αριθμο χωρις κειμενο. Προεπιλογη ειναι `false`
- **isLive** (προαιρετικο): Αν `true`, ο αριθμος θα ενημερωνεται αυτοματα. Προεπιλογη ειναι `false`

## Προχωρημενα Παραδειγματα

### Προσαρμοσμενο URL ID

[inline-code-attrs-start title = 'Comment Count with Custom URL ID'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/widget-comment-count.min.js"></script>
<div id="comment-count-custom"></div>
<script>
    window.FastCommentsCommentCount(document.getElementById('comment-count-custom'), {
        tenantId: 'demo',
        urlId: 'my-custom-page-id'
    });
</script>
[inline-code-end]

### Εμφανιση Μονο Αριθμου

[inline-code-attrs-start title = 'Comment Count Number Only'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/widget-comment-count.min.js"></script>
<div id="comment-count-number"></div>
<script>
    window.FastCommentsCommentCount(document.getElementById('comment-count-number'), {
        tenantId: 'demo',
        numberOnly: true
    });
</script>
[inline-code-end]

### Ζωντανες Ενημερωσεις

[inline-code-attrs-start title = 'Live Comment Count Updates'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/widget-comment-count.min.js"></script>
<div id="comment-count-live"></div>
<script>
    window.FastCommentsCommentCount(document.getElementById('comment-count-live'), {
        tenantId: 'demo',
        isLive: true
    });
</script>
[inline-code-end]

## Μεθοδοι Widget

Το widget επιστρεφει ενα αντικειμενο με τις ακολουθες μεθοδους:

- **destroy()**: Αφαιρει το widget και καθαριζει ολους τους χρονοδιακοπτες
- **update(config)**: Ενημερωνει το widget με νεα διαμορφωση

### Παραδειγμα Χρησης

[inline-code-attrs-start title = 'Widget Methods Example'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/widget-comment-count.min.js"></script>
<div id="comment-count-methods"></div>
<script>
    const widget = window.FastCommentsCommentCount(document.getElementById('comment-count-methods'), {
        tenantId: 'demo'
    });

    // Update the widget to show a different page's count
    setTimeout(() => {
        widget.update({
            tenantId: 'demo',
            urlId: 'different-page-id'
        });
    }, 5000);

    // Destroy the widget after 10 seconds
    setTimeout(() => {
        widget.destroy();
    }, 10000);
</script>
[inline-code-end]

## Στυλ

Το widget αποδιδει απλο HTML με τον αριθμο σχολιων και ερχεται με ελαχιστο στυλ. Μπορειτε να προσαρμοσετε την εμφανιση με CSS:

[inline-code-attrs-start title = 'Custom Styling'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<style>
    .comment-count-styled {
        background: #f0f0f0;
        padding: 5px 10px;
        border-radius: 15px;
        font-size: 14px;
        color: #666;
        display: inline-block;
    }
</style>
<script src="https://cdn.fastcomments.com/js/widget-comment-count.min.js"></script>
<div id="comment-count-styled" class="comment-count-styled"></div>
<script>
    window.FastCommentsCommentCount(document.getElementById('comment-count-styled'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]
