For Step 2 skal vi kopiere vores kodeudsnit. Tjek, at linje 50 ikke siger "demo" - du skal sikre, at dette indeholder your tenant id. Det burde være udfyldt for dig.

Nu kopierer vi vores ThriveCart-Learn-specifikke FastComments-kodeudsnit.

Det er ret stort, fordi integrationen med ThriveCart har mange funktioner, så klik bare på Copy-knappen i øverste højre hjørne af kodeudsnittet:

[inline-code-attrs-start title = 'ThriveCart Learn+ Kommentarer-kode'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
                profileLink = document.querySelector('.thrivecart-courses-header-profile'); // klassen er forskellig i forhåndsvisning.
            }
            // bredt e-mail-input-felt-selektor i tilfælde af, at ThriveCart ændrer id.
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
                // brug rå "img" forespørgsel i tilfælde af, at ThriveCart ændrer billedklasseselektor.
                const avatarImg = profileLink.querySelector('img');
                if (avatarImg && avatarImg.src) {
                    isAuthenticated = true;
                    simpleSSO.avatar = avatarImg.src;
                }
                // brug innerText i tilfælde af, at ThriveCart ændrer, hvordan profilnavnet vises.
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
                // nogle gange bruger TC flere links på samme side, så lad os fjerne dubletter.
                url = getPathnameFromUrl(selectedNavLink.href);
            } else {
                // trim marketing-parametre og domænenavn
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
                // trim marketing-parametre og domænenavn
                return parsedUrl.pathname;
            } catch (error) {
                console.error("Invalid URL", url, error);
                return window.location.pathname; // benyt standard til den aktuelle, så det virker i det mindste nogle gange
            }
        }

    })();
</script>
[inline-code-end]

Nu indsætter du det i kodeblokken til venstre i ThriveCart-editoren. Det burde se sådan ud:

<div class="screenshot white-bg">
    <div class="title">Kode tilføjet</div>
    <img class="screenshot-image" src="/images/installation-guides/thrivecart-learn-step-2-1-paste-code.png" alt="Kode tilføjet" />
</div>

Det var det! Nu skal vi bare udgive:

<div class="screenshot white-bg">
    <div class="title">Udgiv</div>
    <img class="screenshot-image" src="/images/installation-guides/thrivecart-learn-step-2-2-publish.png" alt="Udgiv" />
</div>

Det var det! Du burde nu se kommentarboksen på dit kursus, når du forhåndsviser, og rigtige brugere vil kunne efterlade kommentarer **uden at logge ind eller indtaste deres brugernavn/e-mail igen**.

### Testnote!

Hvis du har anonym kommentering deaktiveret, hvilket den er som standard, vil du ikke kunne efterlade kommentarer i `Preview`-tilstand som `John Smith`-brugeren. Du vil få en autentificeringsfejl, da standardbrugeren `John Smith` ingen e-mail har. Hvis du vil teste, foreslår vi, at du bruger en rabatkode og går igennem dit site som en rigtig bruger.