Za Korak 2 moramo kopirati naš isječak koda. Provjerite da linija 50 ne kaže "demo" - trebali biste osigurati da sadrži vaš tenant id. Trebao bi biti već popunjen za vas.

Sada kopirajmo naš ThriveCart-Learn-specifični FastComments isječak koda.

Prilično je velik, jer integracija s ThriveCart-om ima mnogo funkcija, pa samo kliknite gumb Copy u gornjem desnom kutu isječka koda:

[inline-code-attrs-start title = 'ThriveCart Learn+ Komentari Kod'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
                profileLink = document.querySelector('.thrivecart-courses-header-profile'); // class is different for preview.
            }
            // broad email input field selector incase ThriveCart changes id.
            const emailInputField = document.querySelector('input[type=email]');
            if (emailInputField && emailInputField.value) {
                isAuthenticated = true;
                simpleSSO.email = emailInputField.value;
            } else if (profileLink && !profileLink.innerText.includes('John Smith')) { // allow preview to work - no email available.
                attemptsRemaining--;
                if (!attemptsRemaining) {
                    return console.error('Could not load FastComments - could not determine user information (email). Please reach out to FastComments support.');
                }
                console.warn('FastComments: No user email found - waiting and trying again.');
                return setTimeout(tryLoad, attemptsRemaining < 5 ? 3000 : 100); // increase wait time after 5 attempts incase slow internet.
            }
            if (profileLink) {
                // use raw "img" query incase ThriveCart changes image class selector.
                const avatarImg = profileLink.querySelector('img');
                if (avatarImg && avatarImg.src) {
                    isAuthenticated = true;
                    simpleSSO.avatar = avatarImg.src;
                }
                // use innerText incase ThriveCart changes how profile name is displayed.
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
                return setTimeout(tryLoad, attemptsRemaining < 5 ? 3000 : 100); // increase wait time after 5 attempts incase slow internet.
            }

            let url;
            const selectedNavLink = document.querySelector('.tcc-browse-lesson.active a');

            if (selectedNavLink) {
                // sometimes TC uses multiple links the same page, so let's de-dupe them.
                url = getPathnameFromUrl(selectedNavLink.href);
            } else {
                // trim marketing parameters and domain name
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
                // trim marketing parameters and domain name
                return parsedUrl.pathname;
            } catch (error) {
                console.error("Invalid URL", url, error);
                return window.location.pathname; // default to current, so at least it works sometimes
            }
        }

    })();
</script>
[inline-code-end]

Sada ga zalijepite u blok koda s lijeve strane u ThriveCart uređivaču. Trebalo bi izgledati ovako:

<div class="screenshot white-bg">
    <div class="title">Kod Dodan</div>
    <img class="screenshot-image" src="/images/installation-guides/thrivecart-learn-step-2-1-paste-code.png" alt="Kod Dodan" />
</div>

To je to! Sada samo trebamo objaviti:

<div class="screenshot white-bg">
    <div class="title">Objavi</div>
    <img class="screenshot-image" src="/images/installation-guides/thrivecart-learn-step-2-2-publish.png" alt="Objavi" />
</div>

To je to! Sada biste trebali vidjeti okvir za komentare na svom tečaju kada pregledavate, a stvarni korisnici će moći ostavljati komentare **bez prijave ili ponovnog unosa korisničkog imena/emaila**.

### Napomena o testiranju!

Ako imate onemogućeno anonimno komentiranje, što je zadano, nećete moći ostavljati komentare u `Preview` načinu kao korisnik `John Smith`. Dobit ćete grešku autentifikacije jer zadani korisnik `John Smith` nema email. Ako želite testirati, preporučujemo da koristite kupon kod i prođete kroz svoju stranicu kao stvarni korisnik.