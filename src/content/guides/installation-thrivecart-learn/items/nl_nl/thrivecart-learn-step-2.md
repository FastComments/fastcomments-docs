Voor stap 2 moeten we ons codefragment kopiëren. Controleer dat regel 50 niet "demo" zegt - zorg ervoor dat dit uw tenant id bevat. Het zou voor u ingevuld moeten zijn.

Kopieer nu ons ThriveCart-Learn-specifieke FastComments-codefragment.

Het is vrij omvangrijk, omdat de integratie met ThriveCart veel functies heeft, dus klik gewoon op de Kopiëren-knop rechtsboven in het codefragment:

[inline-code-attrs-start title = 'ThriveCart Learn+ Reactiescode'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
                profileLink = document.querySelector('.thrivecart-courses-header-profile'); // klasse is anders voor preview.
            }
            // brede selector voor e-mailinvoerveld voor het geval ThriveCart de id verandert.
            const emailInputField = document.querySelector('input[type=email]');
            if (emailInputField && emailInputField.value) {
                isAuthenticated = true;
                simpleSSO.email = emailInputField.value;
            } else if (profileLink && !profileLink.innerText.includes('John Smith')) { // laat preview werken - geen e-mailadres beschikbaar.
                attemptsRemaining--;
                if (!attemptsRemaining) {
                    return console.error('Could not load FastComments - could not determine user information (email). Please reach out to FastComments support.');
                }
                console.warn('FastComments: No user email found - waiting and trying again.');
                return setTimeout(tryLoad, attemptsRemaining < 5 ? 3000 : 100); // verhoog de wachttijd na 5 pogingen voor het geval van trage internetverbinding.
            }
            if (profileLink) {
                // gebruik de ruwe "img" query voor het geval ThriveCart de image class-selector verandert.
                const avatarImg = profileLink.querySelector('img');
                if (avatarImg && avatarImg.src) {
                    isAuthenticated = true;
                    simpleSSO.avatar = avatarImg.src;
                }
                // gebruik innerText voor het geval ThriveCart verandert hoe de profielnaam wordt weergegeven.
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
                return setTimeout(tryLoad, attemptsRemaining < 5 ? 3000 : 100); // verhoog de wachttijd na 5 pogingen voor het geval van trage internetverbinding.
            }

            let url;
            const selectedNavLink = document.querySelector('.tcc-browse-lesson.active a');

            if (selectedNavLink) {
                // soms gebruikt TC meerdere links naar dezelfde pagina, dus verwijderen we duplicaten.
                url = getPathnameFromUrl(selectedNavLink.href);
            } else {
                // verwijder marketingparameters en domeinnaam
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
                // verwijder marketingparameters en domeinnaam
                return parsedUrl.pathname;
            } catch (error) {
                console.error("Invalid URL", url, error);
                return window.location.pathname; // standaard naar de huidige locatie, zodat het soms in ieder geval werkt
            }
        }

    })();
</script>
[inline-code-end]

Plak het nu in het codeblok links in de ThriveCart-editor. Het zou er zo uit moeten zien:

<div class="screenshot white-bg">
    <div class="title">Code toegevoegd</div>
    <img class="screenshot-image" src="/images/installation-guides/thrivecart-learn-step-2-1-paste-code.png" alt="Code toegevoegd" />
</div>

Dat is alles! Nu hoeven we het alleen nog te publiceren:

<div class="screenshot white-bg">
    <div class="title">Publiceren</div>
    <img class="screenshot-image" src="/images/installation-guides/thrivecart-learn-step-2-2-publish.png" alt="Publiceren" />
</div>

Dat is alles! Je zou nu het opmerkingenvak op je cursus moeten zien wanneer je een preview bekijkt, en echte gebruikers kunnen opmerkingen achterlaten **zonder opnieuw in te loggen of hun gebruikersnaam/e-mail nog een keer in te voeren**.

### Testopmerking!

Als je anoniem reageren uitgeschakeld hebt, wat standaard het geval is, kun je geen opmerkingen plaatsen in de `Preview`-modus als de gebruiker `John Smith` is. Je krijgt een authenticatiefout omdat de standaardgebruiker `John Smith` geen e-mailadres heeft. Als je wilt testen, raden we aan een kortingscode te gebruiken en je site te doorlopen alsof je een echte gebruiker bent.

---