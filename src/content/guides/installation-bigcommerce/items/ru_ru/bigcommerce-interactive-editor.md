Не рекомендуется добавлять FastComments через Page Builder от BigCommerce, поскольку в этом случае код придётся вручную добавлять на каждую необходимую страницу.

Однако, если это всё же требуется, необходимо использовать следующий фрагмент кода. Фрагменты кода из других руководств не сработают из‑за особенностей BigCommerce:

[inline-code-attrs-start title = 'Сниппет Page Builder для BigCommerce'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: "demo"
    }];
</script>
[inline-code-end]