---
Nie zaleca się dodawania FastComments za pomocą Page Buildera BigCommerce, ponieważ wówczas kod musi być ręcznie dodawany na każdą wybraną stronę.

Jeśli jednak jest to pożądane, należy użyć poniższego fragmentu kodu. Fragmenty kodu z innych samouczków nie będą działać ze względu na specyfikę BigCommerce:

[inline-code-attrs-start title = 'Fragment Page Buildera BigCommerce'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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