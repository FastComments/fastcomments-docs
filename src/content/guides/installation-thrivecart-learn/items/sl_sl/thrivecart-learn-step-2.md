Za korak 2 moramo kopirati naš odsek kode. Preverite, da vrstica 50 ne piše "demo" - želite zagotoviti, da tu stoji vaš tenant id. Moral bi biti že izpolnjen za vas.

Zdaj kopirajmo naš ThriveCart-Learn-specifičen FastComments odsek kode.

It's quite large, because the integration with ThriveCart has a lot of features, so just click the Copy button in the top right of the code snippet:

[inline-code-attrs-start title = 'ThriveCart Learn+ koda komentarjev'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
                profileLink = document.querySelector('.thrivecart-courses-header-profile'); // razred je drugačen za predogled.
            }
            // širok selektor polja e-pošte v primeru, da ThriveCart spremeni id.
            const emailInputField = document.querySelector('input[type=email]');
            if (emailInputField && emailInputField.value) {
                isAuthenticated = true;
                simpleSSO.email = emailInputField.value;
            } else if (profileLink && !profileLink.innerText.includes('John Smith')) { // dovoli delovanje predogleda - e-pošta ni na voljo.
                attemptsRemaining--;
                if (!attemptsRemaining) {
                    return console.error('Could not load FastComments - could not determine user information (email). Please reach out to FastComments support.');
                }
                console.warn('FastComments: No user email found - waiting and trying again.');
                return setTimeout(tryLoad, attemptsRemaining < 5 ? 3000 : 100); // povečaj čas čakanja po 5 poskusih v primeru počasnega interneta.
            }
            if (profileLink) {
                // uporabi neposredno poizvedbo "img" v primeru, da ThriveCart spremeni selektor razreda slike.
                const avatarImg = profileLink.querySelector('img');
                if (avatarImg && avatarImg.src) {
                    isAuthenticated = true;
                    simpleSSO.avatar = avatarImg.src;
                }
                // uporabi innerText v primeru, da ThriveCart spremeni prikaz imena profila.
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
                return setTimeout(tryLoad, attemptsRemaining < 5 ? 3000 : 100); // povečaj čas čakanja po 5 poskusih v primeru počasnega interneta.
            }

            let url;
            const selectedNavLink = document.querySelector('.tcc-browse-lesson.active a');

            if (selectedNavLink) {
                // včasih TC na isti strani uporablja več povezav, zato jih odstranimo podvojitve.
                url = getPathnameFromUrl(selectedNavLink.href);
            } else {
                // odstrani marketinške parametre in ime domene
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
                // odstrani marketinške parametre in ime domene
                return parsedUrl.pathname;
            } catch (error) {
                console.error("Invalid URL", url, error);
                return window.location.pathname; // privzeto na trenutno, tako da vsaj včasih deluje
            }
        }

    })();
</script>
[inline-code-end]

Zdaj ga prilepite v blok kode na levi strani urejevalnika ThriveCart. Izgledalo bi naj takole:

<div class="screenshot white-bg">
    <div class="title">Koda dodana</div>
    <img class="screenshot-image" src="/images/installation-guides/thrivecart-learn-step-2-1-paste-code.png" alt="Koda dodana" />
</div>

To je vse! Zdaj moramo samo še objaviti:

<div class="screenshot white-bg">
    <div class="title">Objavi</div>
    <img class="screenshot-image" src="/images/installation-guides/thrivecart-learn-step-2-2-publish.png" alt="Objavi" />
</div>

To je vse! Zdaj bi morali ob predogledu videti polje za komentarje na vašem tečaju, pravi uporabniki pa bodo lahko pustili komentarje **brez prijave ali ponovnega vnašanja uporabniškega imena/e-pošte**.

### Opomba za testiranje!

Če imate anonimno komentiranje onemogočeno, kar je privzeto, ne boste mogli pustiti komentarjev v `Preview` načinu kot uporabnik `John Smith`. Dobili boste avtentikacijsko
napako, ker privzeti uporabnik `John Smith` nima e-pošte. Če želite testirati, priporočamo, da uporabite kupon in se premikate po svoji strani kot dejanski uporabnik.

---