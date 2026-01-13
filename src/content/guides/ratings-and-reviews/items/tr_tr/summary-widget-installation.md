---
Aşağıda Özet Widget'ı kurmak için Vanilla JS kodu bulunmaktadır. React kütüphanesinde de bu widget mevcuttur.

[inline-code-attrs-start title = 'Özet Widget Kurulumu'; type = 'html'; isFunctional = true; type = 'html';  inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-reviews-summary.min.js"></script>
<div id="summary-widget"></div>
<script>
    window.FastCommentsReviewsSummaryWidget(document.getElementById('summary-widget'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

Widget, sayfa/site için ilgili widget yapılandırmasına göre özet içinde gösterilecek soruları otomatik olarak bulur.

Eğer bu widget'ın bulunmadığı diğer kütüphanelerimizden birinde bu widget'a ihtiyaç duyuyorsanız, eklememiz gerektiğini bilmemiz için bir destek talebi oluşturun.

---