Για το Βήμα 2 πρέπει να αντιγράψουμε το απόσπασμα κώδικα. Ελέγξτε ότι η γραμμή 50 δεν γράφει "demo" - θα θέλετε να βεβαιωθείτε ότι αυτό περιέχει το tenant id σας. Θα πρέπει να έχει συμπληρωθεί για εσάς.

Τώρα ας αντιγράψουμε το ειδικό για ThriveCart Learn απόσπασμα κώδικα του FastComments.

Είναι αρκετά μεγάλο, επειδή η ενσωμάτωση με το ThriveCart έχει πολλές δυνατότητες, οπότε απλώς κάντε κλικ στο κουμπί Copy επάνω δεξιά στο απόσπασμα κώδικα:

[inline-code-attrs-start title = 'Κώδικας σχολίων ThriveCart Learn+'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        let attemptsRemaining = 10;

        function tryLoad() {
            const simpleSSO = {optedInNotifications: true, optedInSubscriptionNotifications: true};
            let isAuthenticated = false;
            let profileLink = document.querySelector('.thrivecart-courses-header-profile-link');
            if (!profileLink) {
                profileLink = document.querySelector('.thrivecart-courses-header-profile'); // η κλάση είναι διαφορετική για την προεπισκόπηση.
            }
            // γενικός επιλογέας πεδίου email σε περίπτωση που το ThriveCart αλλάξει το id.
            const emailInputField = document.querySelector('input[type=email]');
            if (emailInputField && emailInputField.value) {
                isAuthenticated = true;
                simpleSSO.email = emailInputField.value;
            } else if (profileLink && !profileLink.innerText.includes('John Smith')) { // επιτρέψτε στην προεπισκόπηση να λειτουργήσει - δεν υπάρχει διαθέσιμο email.
                attemptsRemaining--;
                if (!attemptsRemaining) {
                    return console.error('Could not load FastComments - could not determine user information (email). Please reach out to FastComments support.');
                }
                console.warn('FastComments: No user email found - waiting and trying again.');
                return setTimeout(tryLoad, attemptsRemaining < 5 ? 3000 : 100); // αύξηση του χρόνου αναμονής μετά από 5 προσπάθειες σε περίπτωση αργής σύνδεσης στο διαδίκτυο.
            }
            if (profileLink) {
                // χρησιμοποιήστε την ακατέργαστη επιλογή "img" σε περίπτωση που το ThriveCart αλλάξει τον επιλεκτή κλάσης της εικόνας.
                const avatarImg = profileLink.querySelector('img');
                if (avatarImg && avatarImg.src) {
                    isAuthenticated = true;
                    simpleSSO.avatar = avatarImg.src;
                }
                // χρησιμοποιήστε innerText σε περίπτωση που το ThriveCart αλλάξει τον τρόπο εμφάνισης του ονόματος προφίλ.
                if (profileLink.innerText) {
                    isAuthenticated = true;
                    simpleSSO.username = profileLink.innerText;
                } else {
                    const bold = profileLink.querySelector('b');
                    if (bold && bold.innerText) {
                        isAuthenticated = true;
                        simpleSSO.username = bold.innerText;
                    }
                }
            } else {
                if (!attemptsRemaining) {
                    return console.error('Could not load FastComments - could not determine user information (user name/avatar). Please reach out to FastComments support.');
                }
                console.warn('FastComments: No user profile info found - waiting and trying again.');
                attemptsRemaining--;
                return setTimeout(tryLoad, attemptsRemaining < 5 ? 3000 : 100); // αύξηση του χρόνου αναμονής μετά από 5 προσπάθειες σε περίπτωση αργής σύνδεσης στο διαδίκτυο.
            }

            let url;
            const selectedNavLink = document.querySelector('.tcc-browse-lesson.active a');

            if (selectedNavLink) {
                // μερικές φορές το TC χρησιμοποιεί πολλούς συνδέσμους στην ίδια σελίδα, οπότε ας αφαιρέσουμε τα διπλότυπα.
                url = getPathnameFromUrl(selectedNavLink.href);
            } else {
                // αφαιρέστε παραμέτρους μάρκετινγκ και το όνομα τομέα
                url = window.location.pathname;
            }

            if (url) {
                url = url.replace('/starte-hier', '');
                url = url.replace('/start-here', '');
            }

            FastCommentsUI(document.getElementById('fastcomments-widget'), {
                tenantId: 'demo',
                urlId: url,
                simpleSSO: isAuthenticated ? simpleSSO : null
            });
        }

        tryLoad();

        function getPathnameFromUrl(url) {
            try {
                const parsedUrl = new URL(url);
                // αφαιρέστε παραμέτρους μάρκετινγκ και το όνομα τομέα
                return parsedUrl.pathname;
            } catch (error) {
                console.error("Invalid URL", url, error);
                return window.location.pathname; // προεπιλογή στην τρέχουσα, έτσι ώστε τουλάχιστον να λειτουργεί μερικές φορές
            }
        }

    })();
</script>
[inline-code-end]

Τώρα επικολλήστε το στο μπλοκ κώδικα στα αριστερά στον επεξεργαστή του ThriveCart. Θα πρέπει να μοιάζει έτσι:

<div class="screenshot white-bg">
    <div class="title">Κώδικας Προστέθηκε</div>
    <img class="screenshot-image" src="/images/installation-guides/thrivecart-learn-step-2-1-paste-code.png" alt="Κώδικας Προστέθηκε" />
</div>

Αυτό ήταν! Τώρα μένει μόνο να δημοσιεύσουμε:

<div class="screenshot white-bg">
    <div class="title">Δημοσίευση</div>
    <img class="screenshot-image" src="/images/installation-guides/thrivecart-learn-step-2-2-publish.png" alt="Δημοσίευση" />
</div>

Αυτό ήταν! Τώρα θα πρέπει να δείτε το πλαίσιο σχολίων στο μάθημά σας όταν κάνετε προεπισκόπηση, και οι πραγματικοί χρήστες θα μπορούν να αφήνουν σχόλια **χωρίς να συνδέονται ή να εισάγουν ξανά το όνομα χρήστη/το email τους**.

### Σημείωση δοκιμής!

Αν έχετε την ανώνυμη δυνατότητα σχολιασμού απενεργοποιημένη, όπως είναι από προεπιλογή, δεν θα μπορείτε να αφήσετε σχόλια σε λειτουργία `Preview` ως χρήστης `John Smith`. Θα λάβετε σφάλμα πιστοποίησης καθώς ο προεπιλεγμένος χρήστης `John Smith` δεν έχει email. Αν θέλετε να δοκιμάσετε, σας προτείνουμε να χρησιμοποιήσετε έναν κωδικό κουπονιού και να περάσετε από τον ιστότοπό σας όπως ένας κανονικός χρήστης.