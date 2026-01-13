Para el Paso 2 tenemos que copiar nuestro fragmento de código. Comprueba que la línea 50 no diga "demo" - querrás asegurarte de que tenga tu tenant id. Debería estar rellenado para ti.

Ahora vamos a copiar nuestro fragmento de código específico de FastComments para ThriveCart Learn.

Es bastante grande, porque la integración con ThriveCart tiene muchas funcionalidades, así que simplemente haz clic en el botón Copiar en la esquina superior derecha del fragmento de código:

[inline-code-attrs-start title = 'Código de comentarios de ThriveCart Learn+'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
                profileLink = document.querySelector('.thrivecart-courses-header-profile'); // la clase es diferente para la vista previa.
            }
            // selector amplio del campo de correo electrónico en caso de que ThriveCart cambie el id.
            const emailInputField = document.querySelector('input[type=email]');
            if (emailInputField && emailInputField.value) {
                isAuthenticated = true;
                simpleSSO.email = emailInputField.value;
            } else if (profileLink && !profileLink.innerText.includes('John Smith')) { // permitir que la vista previa funcione - no hay correo electrónico disponible.
                attemptsRemaining--;
                if (!attemptsRemaining) {
                    return console.error('Could not load FastComments - could not determine user information (email). Please reach out to FastComments support.');
                }
                console.warn('FastComments: No user email found - waiting and trying again.');
                return setTimeout(tryLoad, attemptsRemaining < 5 ? 3000 : 100); // aumentar el tiempo de espera tras 5 intentos en caso de conexión lenta.
            }
            if (profileLink) {
                // usar la consulta 'img' sin procesar en caso de que ThriveCart cambie el selector de la clase de la imagen.
                const avatarImg = profileLink.querySelector('img');
                if (avatarImg && avatarImg.src) {
                    isAuthenticated = true;
                    simpleSSO.avatar = avatarImg.src;
                }
                // usar innerText en caso de que ThriveCart cambie cómo se muestra el nombre del perfil.
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
                return setTimeout(tryLoad, attemptsRemaining < 5 ? 3000 : 100); // aumentar el tiempo de espera tras 5 intentos en caso de conexión lenta.
            }

            let url;
            const selectedNavLink = document.querySelector('.tcc-browse-lesson.active a');

            if (selectedNavLink) {
                // a veces TC usa múltiples enlaces para la misma página, así que los desduplicamos.
                url = getPathnameFromUrl(selectedNavLink.href);
            } else {
                // eliminar parámetros de marketing y el nombre de dominio
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
                // eliminar parámetros de marketing y el nombre de dominio
                return parsedUrl.pathname;
            } catch (error) {
                console.error("Invalid URL", url, error);
                return window.location.pathname; // por defecto a la actual, para que al menos funcione a veces
            }
        }

    })();
</script>
[inline-code-end]

Ahora pégalo en el bloque de código a la izquierda en el editor de ThriveCart. Debería verse así:

<div class="screenshot white-bg">
    <div class="title">Código añadido</div>
    <img class="screenshot-image" src="/images/installation-guides/thrivecart-learn-step-2-1-paste-code.png" alt="Código añadido" />
</div>

¡Eso es todo! Ahora solo tenemos que publicar:

<div class="screenshot white-bg">
    <div class="title">Publicar</div>
    <img class="screenshot-image" src="/images/installation-guides/thrivecart-learn-step-2-2-publish.png" alt="Publicar" />
</div>

¡Eso es todo! Ahora deberías ver el cuadro de comentarios en tu curso cuando lo previsualices, y los usuarios reales podrán dejar comentarios **sin iniciar sesión ni introducir su nombre de usuario/correo electrónico por segunda vez**.

### Nota de prueba!

Si tienes los comentarios anónimos desactivados, que es la configuración por defecto, no podrás dejar comentarios en `Preview` mode as the `John Smith` user. Recibirás un error de autenticación
ya que el usuario por defecto `John Smith` no tiene correo electrónico. Si quieres probar, te sugerimos que uses un código de cupón y navegues por tu sitio como un usuario real.

---