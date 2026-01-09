Za korak 2 moramo kopirati naš isječak koda. Provjerite da na liniji 50 ne piše "demo" - želite osigurati da tu stoji vaš tenant id. Trebalo bi biti popunjeno za vas.

Sada kopirajmo naš ThriveCart-Learn-specifični FastComments isječak koda.

Prilično je velik, jer integracija s ThriveCartom ima puno značajki, pa jednostavno kliknite gumb Copy u gornjem desnom kutu isječka koda:

[inline-code-attrs-start title = 'ThriveCart Learn+ kod komentara'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
                profileLink = document.querySelector('.thrivecart-courses-header-profile'); // klasa je drugačija za preview.
            }
            // široki selektor polja za unos e-pošte u slučaju da ThriveCart promijeni id.
            const emailInputField = document.querySelector('input[type=email]');
            if (emailInputField && emailInputField.value) {
                isAuthenticated = true;
                simpleSSO.email = emailInputField.value;
            } else if (profileLink && !profileLink.innerText.includes('John Smith')) { // dopustiti da preview radi - nema dostupne e-pošte.
                attemptsRemaining--;
                if (!attemptsRemaining) {
                    return console.error('Could not load FastComments - could not determine user information (email). Please reach out to FastComments support.');
                }
                console.warn('FastComments: No user email found - waiting and trying again.');
                return setTimeout(tryLoad, attemptsRemaining < 5 ? 3000 : 100); // povećaj vrijeme čekanja nakon 5 pokušaja u slučaju sporog interneta.
            }
            if (profileLink) {
                // koristi sirovu "img" upit u slučaju da ThriveCart promijeni selektor klase slike.
                const avatarImg = profileLink.querySelector('img');
                if (avatarImg && avatarImg.src) {
                    isAuthenticated = true;
                    simpleSSO.avatar = avatarImg.src;
                }
                // koristi innerText u slučaju da ThriveCart promijeni način prikaza imena profila.
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
                return setTimeout(tryLoad, attemptsRemaining < 5 ? 3000 : 100); // povećaj vrijeme čekanja nakon 5 pokušaja u slučaju sporog interneta.
            }

            let url;
            const selectedNavLink = document.querySelector('.tcc-browse-lesson.active a');

            if (selectedNavLink) {
                // ponekad TC koristi više poveznica na istoj stranici, pa ih uklonimo kako bismo izbjegli duplikate.
                url = getPathnameFromUrl(selectedNavLink.href);
            } else {
                // obreži marketinške parametre i naziv domene
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
                // obreži marketinške parametre i naziv domene
                return parsedUrl.pathname;
            } catch (error) {
                console.error("Invalid URL", url, error);
                return window.location.pathname; // zadano na trenutačnu stranicu, tako da barem ponekad radi
            }
        }

    })();
</script>
[inline-code-end]

Sada ga zalijepite u blok koda s lijeve strane u ThriveCart editoru. Trebalo bi izgledati ovako:

<div class="screenshot white-bg">
    <div class="title">Dodani kod</div>
    <img class="screenshot-image" src="/images/installation-guides/thrivecart-learn-step-2-1-paste-code.png" alt="Dodani kod" />
</div>

To je to! Sad samo trebamo objaviti:

<div class="screenshot white-bg">
    <div class="title">Objavi</div>
    <img class="screenshot-image" src="/images/installation-guides/thrivecart-learn-step-2-2-publish.png" alt="Objavi" />
</div>

To je to! Sada biste trebali vidjeti okvir za komentare na svom tečaju pri pregledavanju, a stvarni korisnici moći će ostavljati komentare **bez ponovnog prijavljivanja ili ponovnog unošenja svog korisničkog imena/e-pošte**.

### Testing Note!

Ako imate onemogućeno anonimno komentiranje, što je zadano, nećete moći ostaviti komentare u `Preview` načinu rada kao korisnik `John Smith`. Dobit ćete grešku autentifikacije jer zadani korisnik `John Smith` nema e-poštu. Ako želite testirati, predlažemo da upotrijebite kupon i prođete kroz svoj site kao stvarni korisnik.