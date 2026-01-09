Now that we're in the template editor, we must decide where we want to display the comments, or live chat.

In this example we will add it directly below the video. Hover the element to add the widget to the end of, and click `ADD ELEMENT`:

<div class="screenshot white-bg">
    <div class="title">Öğe Ekle</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-4-add-element.png" alt="Öğe Ekle" />
</div>

Select `CUSTOM JS/HTML`:

<div class="screenshot white-bg">
    <div class="title">CUSTOM JS/HTML'i Seçin</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-5-custom-js-html.png" alt="CUSTOM JS/HTML'i Seçin" />
</div>

Now let's open the code editor where we'll paste our code.

ClickFunnels is a bit confusing on this next step.

It's important that you *DO NOT* select `Code` when you hover over the new element. Instead, select `SETTINGS`:

<div class="screenshot white-bg">
    <div class="title">SETTINGS'i Seçin</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-6-settings.png" alt="SETTINGS'i Seçin" />
</div>

Now on the right hand side we can click `Open Code Editor`:

<div class="screenshot white-bg">
    <div class="title">Open Code Editor'a Tıklayın</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-7-open-code-editor.png" alt="Open Code Editor'a Tıklayın" />
</div>

You'll see a big square open up. This is where we can paste our code. Copy the following snippet (use the copy button in the top right):

[inline-code-attrs-start title = 'ClickFunnels Akış Sohbeti Kod Parçacığı'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-live-chat.min.js"></script>
<div id="fastcomments-live-chat-widget" style="width: 500px;min-height: 780px;"></div>
<style>
    #fastcomments-live-chat-widget iframe {
        min-height: 780px;
    }
</style>
<script>
    (function fcLoad() {
        function tryLoad() {
            // bazı sağlayıcılar kod parçacığını async hale getiriyor
            const container = document.getElementById('fastcomments-live-chat-widget');
            if (!container) {
                return waitRetry();
            }
            if (!window.FastCommentsLiveChat) {
                return waitRetry();
            }
            window.FastCommentsLiveChat(container, {
                tenantId: 'demo'
            });
        }
        function waitRetry() {
            setTimeout(tryLoad, 500);
        }
        tryLoad();
    })();
</script>
[inline-code-end]

This code snippet is for our Streaming Chat product, which goes well with videos. If you want the Live Commenting widget code snippet instead, which
goes best with regular pages or blog posts, it's at the end of this tutorial.

When we paste the code snippet into the window, it should look like this:

<div class="screenshot white-bg">
    <div class="title">Kod Yapıştır</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-8-paste.png" alt="Kod Yapıştır" />
</div>

Now we just have to close the box:

<div class="screenshot white-bg">
    <div class="title">Kapat</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-9-close.png" alt="Kapat" />
</div>

Now you can preview your changes! Feel free to move the widget around and see where you like it best.

<div class="screenshot white-bg">
    <div class="title">Önizleme</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-10-preview.png" alt="Önizleme" />
</div>

Success! Don't forget to test mobile!

<div class="screenshot white-bg">
    <div class="title">Başarılı!</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-11-success.png" alt="Başarılı!" />
</div>