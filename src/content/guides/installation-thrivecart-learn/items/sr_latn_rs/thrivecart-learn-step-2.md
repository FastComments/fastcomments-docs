Za korak 2 moramo kopirati naš kod. Proverite da linija 50 ne sadrži "demo" - želite da budete sigurni da ovo sadrži vaš tenant id. Trebalo bi da bude popunjeno za vas.

Sada kopirajmo naš ThriveCart-Learn-specifični FastComments kod.

On je prilično velik, jer integracija sa ThriveCart ima mnogo funkcija, pa samo kliknite dugme Copy u gornjem desnom uglu bloka koda:

[inline-code-attrs-start title = 'ThriveCart Learn+ kod za komentare'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
                profileLink = document.querySelector('.thrivecart-courses-header-profile'); // klasa je drugačija za pregled.
            }
            // širok selektor polja za email u slučaju da ThriveCart promeni id.
            const emailInputField = document.querySelector('input[type=email]');
            if (emailInputField && emailInputField.value) {
                isAuthenticated = true;
                simpleSSO.email = emailInputField.value;
            } else if (profileLink && !profileLink.innerText.includes('John Smith')) { // dozvoli da pregled funkcioniše - nema dostupnog emaila.
                attemptsRemaining--;
                if (!attemptsRemaining) {
                    return console.error('Could not load FastComments - could not determine user information (email). Please reach out to FastComments support.');
                }
                console.warn('FastComments: No user email found - waiting and trying again.');
                return setTimeout(tryLoad, attemptsRemaining < 5 ? 3000 : 100); // povećaj vreme čekanja nakon 5 pokušaja u slučaju sporog interneta.
            }
            if (profileLink) {
                // koristi raw "img" query incase ThriveCart changes image class selector.
                const avatarImg = profileLink.querySelector('img');
                if (avatarImg && avatarImg.src) {
                    isAuthenticated = true;
                    simpleSSO.avatar = avatarImg.src;
                }
                // koristi innerText incase ThriveCart changes how profile name is displayed.
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
                return setTimeout(tryLoad, attemptsRemaining < 5 ? 3000 : 100); // povećaj vreme čekanja nakon 5 pokušaja u slučaju sporog interneta.
            }

            let url;
            const selectedNavLink = document.querySelector('.tcc-browse-lesson.active a');

            if (selectedNavLink) {
                // ponekad TC koristi više linkova na istoj stranici, pa ih de-duplikujemo.
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

Sada ga nalepite u blok za kod levo u ThriveCart editoru. Trebalo bi da izgleda ovako:

<div class="screenshot white-bg">
    <div class="title">Kod dodat</div>
    <img class="screenshot-image" src="/images/installation-guides/thrivecart-learn-step-2-1-paste-code.png" alt="Kod dodat" />
</div>

To je to! Sada samo treba da objavimo:

<div class="screenshot white-bg">
    <div class="title">Objavi</div>
    <img class="screenshot-image" src="/images/installation-guides/thrivecart-learn-step-2-2-publish.png" alt="Objavi" />
</div>

To je to! Sada bi trebalo da vidite polje za komentare na vašem kursu kada pregledate, i stvarni korisnici će moći da ostavljaju komentare **bez prijavljivanja ili ponovnog unošenja svog korisničkog imena/emaila**.

### Napomena za testiranje!

Ako imate onemogućen anonimni komentar, što je podrazumevano, nećete moći da ostavljate komentare u `Preview` režimu kao korisnik `John Smith`. Dobit ćete grešku pri autentifikaciji jer podrazumevani korisnik `John Smith` nema email. Ako želite da testirate, predlažemo da koristite kupon kod i prođete kroz svoj sajt kao pravi korisnik.