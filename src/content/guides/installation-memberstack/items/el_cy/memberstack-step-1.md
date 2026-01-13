Μπορούμε εύκολα να συνδέσουμε το FastComments με το Memberstack με ένα μικρό απόσπασμα κώδικα:

[inline-code-attrs-start title = 'FastComments Memberstack Snippet'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    window.$memberstackDom.getCurrentMember().then(({data: member}) => {
        if (member) {
            FastCommentsUI(document.getElementById('fastcomments-widget'), {
                tenantId: "demo",
                urlId: window.location.pathname,
                simpleSSO: {
                    username: member.customFields.name || member.auth.email.replace(/@.+/, ''),
                    email: member.auth.email,
                    avatar: member.customFields.avatar
                }
            });
        } else {
            FastCommentsUI(document.getElementById('fastcomments-widget'), {
                tenantId: "demo",
                urlId: window.location.pathname,
                simpleSSO: null
            });
        }
    })
</script>
[inline-code-end]

Όταν ο χρήστης επισκέπτεται τον ιστότοπο ή την εφαρμογή σας ενώ είναι συνδεδεμένος μέσω Memberstack, θα συνδεθεί αυτόματα στο FastComments και τα σχόλιά του
θα επισημανθούν ως `Verified`.

**Θα μπορούν επίσης να σχολιάσουν αφήνοντας το όνομα και το email τους, αν δεν είναι συνδεδεμένοι.**

### Σχολιασμός μόνο για μέλη

Αν θέλετε να έχετε **σχολιασμό μόνο για μέλη**, δοκιμάστε το ακόλουθο απόσπασμα κώδικα, αλλά αλλάξτε το `https://example.com/login` στο σημείο όπου θέλετε να πάνε οι χρήστες όταν κάνουν κλικ στο κουμπί `Login`:

[inline-code-attrs-start title = 'Exclusive FastComments Memberstack Snippet'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    window.$memberstackDom.getCurrentMember().then(({data: member}) => {
        if (member) {
            FastCommentsUI(document.getElementById('fastcomments-widget'), {
                tenantId: "demo",
                urlId: window.location.pathname,
                simpleSSO: {
                    username: member.customFields.name || member.auth.email.replace(/@.+/, ''),
                    email: member.auth.email,
                    avatar: member.customFields.avatar
                }
            });
        } else {
            FastCommentsUI(document.getElementById('fastcomments-widget'), {
                tenantId: "demo",
                urlId: window.location.pathname,
                simpleSSO: {
                    loginURL: 'https://example.com/login'
                }
            });
        }
    })
</script>
[inline-code-end]
