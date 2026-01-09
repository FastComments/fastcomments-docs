Pour l'étape 2, nous devons copier notre extrait de code. Vérifiez que la ligne 50 ne contient pas "demo" - vous devrez vous assurer qu'elle contient votre tenant id. Il devrait être rempli pour vous.

Copions maintenant notre extrait de code FastComments spécifique à ThriveCart Learn.

Il est assez volumineux, car l'intégration avec ThriveCart offre beaucoup de fonctionnalités, alors cliquez simplement sur le bouton Copier en haut à droite de l'extrait de code :

[inline-code-attrs-start title = 'Code des commentaires ThriveCart Learn+'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
                profileLink = document.querySelector('.thrivecart-courses-header-profile'); // la classe est différente pour l'aperçu.
            }
            // sélecteur large pour le champ d'adresse courriel au cas où ThriveCart changerait l'id.
            const emailInputField = document.querySelector('input[type=email]');
            if (emailInputField && emailInputField.value) {
                isAuthenticated = true;
                simpleSSO.email = emailInputField.value;
            } else if (profileLink && !profileLink.innerText.includes('John Smith')) { // permettre à l'aperçu de fonctionner — pas d'email disponible.
                attemptsRemaining--;
                if (!attemptsRemaining) {
                    return console.error('Could not load FastComments - could not determine user information (email). Please reach out to FastComments support.');
                }
                console.warn('FastComments: No user email found - waiting and trying again.');
                return setTimeout(tryLoad, attemptsRemaining < 5 ? 3000 : 100); // augmenter le temps d'attente après 5 tentatives au cas où la connexion serait lente.
            }
            if (profileLink) {
                // utiliser la requête brute "img" au cas où ThriveCart changerait le sélecteur de classe d'image.
                const avatarImg = profileLink.querySelector('img');
                if (avatarImg && avatarImg.src) {
                    isAuthenticated = true;
                    simpleSSO.avatar = avatarImg.src;
                }
                // utiliser innerText au cas où ThriveCart modifierait la façon dont le nom du profil est affiché.
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
                return setTimeout(tryLoad, attemptsRemaining < 5 ? 3000 : 100); // augmenter le temps d'attente après 5 tentatives au cas où la connexion serait lente.
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
                return window.location.pathname; // valeur par défaut sur la page actuelle, pour que ça fonctionne au moins parfois
            }
        }

    })();
</script>
[inline-code-end]

Collez-le maintenant dans le bloc de code à gauche de l'éditeur ThriveCart. Cela devrait ressembler à ceci :

<div class="screenshot white-bg">
    <div class="title">Code ajouté</div>
    <img class="screenshot-image" src="/images/installation-guides/thrivecart-learn-step-2-1-paste-code.png" alt="Code ajouté" />
</div>

C'est tout ! Maintenant, il ne nous reste plus qu'à publier :

<div class="screenshot white-bg">
    <div class="title">Publier</div>
    <img class="screenshot-image" src="/images/installation-guides/thrivecart-learn-step-2-2-publish.png" alt="Publier" />
</div>

C'est tout ! Vous devriez maintenant voir la boîte de commentaires sur votre cours lorsque vous prévisualisez, et les utilisateurs réels pourront laisser des commentaires **sans se connecter ni ressaisir leur nom d'utilisateur/courriel**.

### Remarque pour les tests !

Si les commentaires anonymes sont désactivés, ce qui est le cas par défaut, vous ne pourrez pas laisser de commentaires en mode `Preview` en tant qu'utilisateur `John Smith`. Vous obtiendrez une erreur d'authentification
car l'utilisateur par défaut `John Smith` n'a pas de courriel. Si vous souhaitez tester, nous vous suggérons d'utiliser un code promo et de parcourir votre site comme un utilisateur réel.