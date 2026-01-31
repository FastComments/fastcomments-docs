Не рекомендується додавати FastComments через BigCommerce Page Builder, оскільки в такому випадку код доведеться додавати вручну на кожну потрібну сторінку.

Однак, якщо це бажано, потрібно використати наступний фрагмент коду. Фрагменти коду з інших підручників не працюватимуть через особливості BigCommerce:

[inline-code-attrs-start title = 'Фрагмент для BigCommerce Page Builder'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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