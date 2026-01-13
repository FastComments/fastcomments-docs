Η εγκατάσταση είναι απλή:

[inline-code-attrs-start title = 'Παράδειγμα κώδικα Floating Likes'; type = 'html'; isFunctional = false; type = 'html';  inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-page-likes-floating.min.js"></script>
<div id="fastcomments-page-likes-floating"></div>
<script>
    window.FastCommentsEmbedPageLikesFloating(document.getElementById('fastcomments-page-likes-floating'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

Η υπογραφή τύπου του constructor είναι:

[inline-code-attrs-start title = 'Διαμόρφωση'; useDemoTenant = true; isFunctional = false; type = 'javascript';  inline-code-attrs-end]
[inline-code-start]
/**
 *
 * @param {HTMLElement} targetElement
 * @param config
 * @property {string} tenantId
 * @property {string} urlId - Αλλάξτε αυτό για να ορίσετε το id της σελίδας/άρθρου. Από προεπιλογή, είναι το URL της σελίδας.
 * @property {() => void} [onOpenComments]
 * @property {object} [sso]
 * @constructor
 */
[inline-code-end]

Υποστηρίζει sso για να συνδέει τις αντιδράσεις με τον λογαριασμό του χρήστη για σκοπούς αυθεντικοποίησης.

Προς το παρόν, υποστηρίζεται μόνο VanillaJS. Αν θέλετε αυτό το widget να προστεθεί σε μία από τις βιβλιοθήκες πελάτη μας, ενημερώστε μας! 

### Ασύγχρονη Έκδοση

[inline-code-attrs-start title = 'Παράδειγμα ασύγχρονου κώδικα Floating Likes'; isFunctional = false; type = 'html';  inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-page-likes-floating.min.js?v=2" async></script>
<div id="fastcomments-page-likes-floating"></div>
<script>
    (function () {
        function tryLoad() {
            if (window.FastCommentsEmbedPageLikesFloating) {
                window.FastCommentsEmbedPageLikesFloating(document.getElementById('fastcomments-page-likes-floating'), {
                    tenantId: 'demo'
                });
            } else {
                setTimeout(tryLoad, 50);
            }
        }

        tryLoad();
    })();
</script>
[inline-code-end]

### Με SSO

Μπορούμε επίσης να περάσουμε φορτία Secure SSO ή Simple SSO:

[inline-code-attrs-start title = 'Παράδειγμα κώδικα Floating Likes με Ασφαλές SSO'; type = 'html'; isFunctional = false; type = 'html';  inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-page-likes-floating.min.js"></script>
<div id="fastcomments-page-likes-floating"></div>
<script>
    window.FastCommentsEmbedPageLikesFloating(document.getElementById('fastcomments-page-likes-floating'), {
        tenantId: 'demo',
        sso // περάστε το αντικείμενο sso
    });
</script>
[inline-code-end]

[inline-code-attrs-start title = 'Παράδειγμα κώδικα Floating Likes με Απλό SSO'; type = 'html'; isFunctional = false; type = 'html';  inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-page-likes-floating.min.js"></script>
<div id="fastcomments-page-likes-floating"></div>
<script>
    window.FastCommentsEmbedPageLikesFloating(document.getElementById('fastcomments-page-likes-floating'), {
        tenantId: 'demo',
        simpleSSO: {
            id: 'some-user-id',
            username: 'some username',
            email: 'some@email.com',
        }
    });
</script>
[inline-code-end]