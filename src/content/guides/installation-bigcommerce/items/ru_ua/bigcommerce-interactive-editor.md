---
Не рекомендуется добавлять FastComments через конструктор страниц (Page Builder) BigCommerce, так как в этом случае код придётся вручную добавлять на каждую требуемую страницу.

Однако, если это необходимо, нужно использовать следующий фрагмент кода. Фрагменты кода из других руководств не будут работать из‑за особенностей BigCommerce:

[inline-code-attrs-start title = 'Фрагмент конструктора страниц BigCommerce'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

---