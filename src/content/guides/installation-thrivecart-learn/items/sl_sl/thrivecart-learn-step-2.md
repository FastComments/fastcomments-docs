Za 2. korak moramo kopirati našo kodo. Preverite, da vrstica 50 ne pravi "demo" – poskrbeli boste, da bo tam vaš ID najemnika (tenant id). Ta bi moral biti že izpolnjen za vas.

Zdaj kopirajmo našo kodo, specifično za ThriveCart-Learn.

Je precej obsežna, ker ima integracija s ThriveCart veliko funkcionalnosti, zato preprosto kliknite gumb Copy v zgornjem desnem kotu kode:

[inline-code-attrs-start title = 'Koda za komentarje ThriveCart Learn+'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Zdaj jo prilepite v blok kode na levi strani v ThriveCart urejevalniku. Videti mora biti takole:

<div class="screenshot white-bg">
    <div class="title">Koda dodana</div>
    <img class="screenshot-image" src="/images/installation-guides/thrivecart-learn-step-2-1-paste-code.png" alt="Koda dodana" />
</div>

To je vse! Zdaj moramo le še objaviti:

<div class="screenshot white-bg">
    <div class="title">Objava</div>
    <img class="screenshot-image" src="/images/installation-guides/thrivecart-learn-step-2-2-publish.png" alt="Objava" />
</div>

To je to! Zdaj bi morali pri predogledu videti okvir za komentarje v vašem tečaju, pravi uporabniki pa bodo lahko puščali komentarje **brez prijave ali ponovnega vnašanja uporabniškega imena/e-pošte**.

### Opomba za testiranje!

Če imate onemogočeno anonimno komentiranje (kar je privzeto), ne boste mogli puščati komentarjev v načinu `Preview` kot uporabnik `John Smith`. Prejeli boste napako avtentikacije, saj privzeti uporabnik `John Smith` nima e-pošte. Če želite testirati, priporočamo, da uporabite kodo za kupon in prehodite svojo stran kot pravi uporabnik.