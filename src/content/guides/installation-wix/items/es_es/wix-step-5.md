Next, let's set things up so that the comment thread changes based on the current page, allowing users to discuss the currently displayed content.

Without the following steps, you will only have one global comment thread for your entire site - which is not very useful.

#### Dev Mode

To add this functionality, we'll have to go into what Wix calls `Dev Mode`.

Click the `Dev Mode` option at the top of the screen.

<div class="screenshot white-bg">
    <div class="title">Activar Dev Mode</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-dev-mode.png" alt="Activar Dev Mode" />
</div>

#### Set the Element ID

We're going to add custom code to accomplish this, but first we need to give the new embed element an ID to refer to it by.

Let's call it `fastcomments`.

Click the new embed element we added, and in dev mode in the bottom right you should see an ID field with a value like `html1`:

<div class="screenshot white-bg">
    <div class="title">El campo ID</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-id-0.png" alt="El campo ID" />
</div>

Change this to `fastcomments` and hit enter:

<div class="screenshot white-bg">
    <div class="title">Establecer el ID</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-id-1.png" alt="Establecer el ID" />
</div>

Now we can add our custom code that tells the comment area what page we are viewing.

At the bottom of the screen you should see a code editor like this:

<div class="screenshot white-bg">
    <div class="title">Abrir el editor</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-7-open-editor.png" alt="Abrir el editor" />
</div>

Copy the following code and paste it in there:

[inline-code-attrs-start title = 'Fragmento de navegación de Wix Comments'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
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
    <div class="title">Añadir el código de navegación</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-7-paste-code.png" alt="Añadir el código de navegación" />
</div>

---