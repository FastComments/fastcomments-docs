Μπορούμε εύκολα να συνδέσουμε το FastComments με το MemberSpace με ένα μικρό απόσπασμα κώδικα:

[inline-code-attrs-start title = 'FastComments MemberSpace Snippet'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        const tenantId = 'demo';
        const ALLOW_ANON = false;
        const LOGIN_URL = 'https://example.com/login';
        const PLAN_DISPLAY_LABELS = {
            'VIP Plan': 'VIP'
        };
        let lastInstance;

        function tick() {
            if (!window.MemberSpace) {
                return setTimeout(tick, 100);
            }
            if (!window.FastCommentsUI) {
                return setTimeout(tick, 100);
            }
            const target = document.getElementById('fastcomments-widget');
            if (!target) {
                return setTimeout(tick, 100);
            }
            const data = MemberSpace.getMemberInfo();
            if (data.isLoggedIn && data.memberInfo) {
                lastInstance = FastCommentsUI(target, {
                    tenantId: tenantId,
                    urlId: window.location.pathname,
                    simpleSSO: {
                        displayLabel: getDisplayLabel(data.memberInfo),
                        username: data.memberInfo.name,
                        email: data.memberInfo.email,
                        avatar: data.memberInfo.profileImageUrl
                    }
                });
            } else if (lastInstance) {
                lastInstance.destroy();
                lastInstance = FastCommentsUI(target, {
                    tenantId: tenantId,
                    urlId: window.location.pathname,
                    simpleSSO: getAnonSSOConfig()
                });
            }
        }

        function getAnonSSOConfig() {
            if (ALLOW_ANON) {
                return undefined;
            }
            return {
                loginURL: LOGIN_URL
            };
        }

        function getDisplayLabel(memberInfo) {
            if (!memberInfo.memberships) {
                return;
            }
            for (const membership of memberInfo.memberships) {
                const label = PLAN_DISPLAY_LABELS[membership.name];
                if (label) {
                    return label
                }
            }
        }

        tick();
    })();
</script>
[inline-code-end]

Όταν ο χρήστης επισκέπτεται τον ιστότοπο ή την εφαρμογή σας ενώ είναι συνδεδεμένος μέσω MemberStack, θα συνδεθεί αυτόματα στο FastComments και τα σχόλιά του
θα επισημανθούν ως `Verified`.

Επιπλέον, στο παραπάνω παράδειγμα, αν έχετε ένα πρόγραμμα συνδρομής που ονομάζεται `VIP Plan`, θα εμφανίσουμε ένα σήμα `VIP` δίπλα στο όνομα του χρήστη. Μπορείτε να επεξεργαστείτε το παράδειγμα για να
προσθέσετε περισσότερα προγράμματα. Επικοινωνήστε με την υποστήριξη αν έχετε ερωτήσεις.

### Επιτρέψτε ανώνυμα σχόλια

Αν θέλετε επίσης να έχετε **ανώνυμα σχόλια**, ορίστε το ALLOW_ANON σε true ως εξής:

    const ALLOW_ANON = true;

Επίσης, θυμηθείτε να αλλάξετε το `https://example.com/login` στο σημείο όπου θέλετε να πηγαίνουν οι χρήστες όταν κάνουν κλικ στο κουμπί `Login`:

Με αυτόν τον τρόπο οι χρήστες θα έχουν τη δυνατότητα να εισάγουν το όνομα και το email τους για να σχολιάσουν αν δεν είναι συνδεδεμένοι στον ιστότοπο μελών σας.
