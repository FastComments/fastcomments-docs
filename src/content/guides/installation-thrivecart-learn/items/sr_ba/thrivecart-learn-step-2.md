Za korak 2 moramo kopirati naš isječak koda. Provjerite da linija 50 ne piše "demo" - želite se pobrinuti da tu stoji vaš tenant id. Trebalo bi da je već popunjeno za vas.

Sada kopirajmo naš ThriveCart-Learn-specifični FastComments isječak koda.

Prilično je velik, jer integracija sa ThriveCart ima puno funkcija, pa jednostavno kliknite gumb Copy u gornjem desnom kutu isječka koda:

[inline-code-attrs-start title = 'ThriveCart Learn+ Kod komentara'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
            // širok selektor polja za unos emaila u slučaju da ThriveCart promijeni id.
            const emailInputField = document.querySelector('input[type=email]');
            if (emailInputField && emailInputField.value) {
                isAuthenticated = true;
                simpleSSO.email = emailInputField.value;
            } else if (profileLink && !profileLink.innerText.includes('John Smith')) { // dopusti da pregled radi - nema dostupnog emaila.
                attemptsRemaining--;
                if (!attemptsRemaining) {
                    return console.error('Could not load FastComments - could not determine user information (email). Please reach out to FastComments support.');
                }
                console.warn('FastComments: No user email found - waiting and trying again.');
                return setTimeout(tryLoad, attemptsRemaining < 5 ? 3000 : 100); // increase wait time after 5 attempts incase slow internet.
            }
            if (profileLink) {
                // koristi sirovu "img" selekciju u slučaju da ThriveCart promijeni klasu za sliku.
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
                return setTimeout(tryLoad, attemptsRemaining < 5 ? 3000 : 100); // increase wait time after 5 attempts incase slow internet.
            }

            let url;
            const selectedNavLink = document.querySelector('.tcc-browse-lesson.active a');

            if (selectedNavLink) {
                // ponekad TC koristi više istih linkova na istoj stranici, pa ih treba deduplicirati.
                url = getPathnameFromUrl(selectedNavLink.href);
            } else {
                // ukloni marketinške parametre i ime domena
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
                // ukloni marketinške parametre i ime domena
                return parsedUrl.pathname;
            } catch (error) {
                console.error("Invalid URL", url, error);
                return window.location.pathname; // default to current, so at least it works sometimes
            }
        }

    })();
</script>
[inline-code-end]

Sada ga zalijepite u blok koda s lijeve strane u ThriveCart editoru. Trebalo bi da izgleda ovako:

<div class="screenshot white-bg">
    <div class="title">Kod dodat</div>
    <img class="screenshot-image" src="/images/installation-guides/thrivecart-learn-step-2-1-paste-code.png" alt="Kod dodat" />
</div>

To je to! Sada samo moramo objaviti:

<div class="screenshot white-bg">
    <div class="title">Objavi</div>
    <img class="screenshot-image" src="/images/installation-guides/thrivecart-learn-step-2-2-publish.png" alt="Objavi" />
</div>

To je to! Sada biste trebali vidjeti polje za komentare na vašem kursu kada ga pregledate, a stvarni korisnici će moći ostaviti komentare **bez prijavljivanja ili ponovnog unošenja korisničkog imena/e-maila**.

### Napomena za testiranje!

Ako imate anonimno komentarisanje onemogućeno, što je po defaultu, nećete moći ostaviti komentare u `Preview` modu kao korisnik `John Smith`. Dobit ćete grešku autentifikacije jer zadani korisnik `John Smith` nema email. Ako želite testirati, predlažemo da upotrijebite kupon kod i prođete kroz svoj sajt kao stvarni korisnik.

---