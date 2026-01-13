Now that you've added a custom HTML block, we can add the FastComments widget code.

**Utilisez le code suivant pour Godaddy, pas le code provenant d'autres tutoriels. Ce code est spécifique à Godaddy.**

Copiez le code suivant:

[inline-code-attrs-start title = 'Extrait de code des commentaires Godaddy'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        if (top.location.pathname && top.location.pathname.includes('/f')) {
            FastCommentsUI(document.getElementById('fastcomments-widget'), {
                tenantId: "demo",
                pageTitle: top.document.title,
                url: top.location.href,
                urlId: top.location.pathname
            });
        }
    })();
</script>
[inline-code-end]

This specific code snippet is designed to be compatible with Godaddy, and will also only show on your blog posts - not the homepage.

Now paste the code into the `Custom Code` area mentioned in `Step One`.

<div class="screenshot white-bg">
    <div class="title">Copiez et collez le code</div>
    <img class="screenshot-image" src="/images/installation-guides/godaddy-step-2-code-added.png" alt="Copiez et collez le code" />
</div>

Hit Done in the top right:

<div class="screenshot white-bg">
    <div class="title">Cliquez sur Terminé</div>
    <img class="screenshot-image" src="/images/installation-guides/godaddy-step-2-done.png" alt="Cliquez sur Terminé" />
</div>

That's it for step two!

---