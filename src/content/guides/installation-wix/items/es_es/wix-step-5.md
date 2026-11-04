A continuación, configuremos las cosas para que el hilo de comentarios cambie según la página actual, permitiendo que los usuarios discutan el contenido que se muestra en ese momento.

Sin los siguientes pasos, solo tendrás un hilo de comentarios global para todo tu sitio, lo cual no es muy útil.

#### Modo de Desarrollo

Para agregar esta funcionalidad, tendremos que entrar en lo que Wix llama `Dev Mode`.

Haz clic en la opción `Dev Mode` en la parte superior de la pantalla.

<div class="screenshot white-bg">
    <div class="title">Habilitar el Modo de Desarrollo</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-dev-mode.png" alt="Habilitar el Modo de Desarrollo" />
</div>

#### Establecer el ID del Elemento

Vamos a agregar código personalizado para lograr esto, pero primero necesitamos darle al nuevo elemento de inserción un ID para poder referirnos a él.

Llamémoslo `fastcomments`.

Haz clic en el nuevo elemento de inserción que agregamos, y en el modo de desarrollo, en la parte inferior derecha deberías ver un campo de ID con un valor como `html1`:

<div class="screenshot white-bg">
    <div class="title">El Campo de ID</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-id-0.png" alt="El Campo de ID" />
</div>

Cámbialo a `fastcomments` y presiona enter:

<div class="screenshot white-bg">
    <div class="title">Establecer el ID</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-id-1.png" alt="Establecer el ID" />
</div>

Ahora podemos agregar nuestro código personalizado que le indica al área de comentarios qué página estamos viendo.

En la parte inferior de la pantalla deberías ver un editor de código como este:

<div class="screenshot white-bg">
    <div class="title">Abrir el Editor</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-7-open-editor.png" alt="Establecer el ID" />
</div>

Copia el siguiente código y pégalo ahí:

[inline-code-attrs-start title = 'Fragmento de Navegación de Comentarios de Wix'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import wixLocation from 'wix-location';

$w.onReady(function () {
    function updateFastCommentsLocation() {
        try {
            const url = (wixLocation.baseUrl + '/' + wixLocation.path).replace(/,/g, '/');
            $w('#fastcomments').postMessage({'action': 'reload', 'url': url});
        } catch (err) {
            console.error('Wix -> FastComments Error', err);
        }
    }

    updateFastCommentsLocation();
    wixLocation.onChange( () => {
        updateFastCommentsLocation();
    });
});
[inline-code-end]

<div class="screenshot white-bg">
    <div class="title">Agregar el Código de Navegación</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-7-paste-code.png" alt="Agregar el Código de Navegación" />
</div>