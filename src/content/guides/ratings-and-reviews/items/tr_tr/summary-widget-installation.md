Aşağıda, Özet Widget'ını kurmak için gereken Vanilla JS kodu yer almaktadır. React kütüphanesinde de bu widget bulunmaktadır.

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

Widget, ilgili sayfa/site için karşılık gelen widget yapılandırmasına dayanarak özette gösterilecek soruları otomatik olarak bulacaktır.

### Yorumları Puana Göre Filtreleme

Aynı sayfanın yorum widget'ı da sayfada bulunuyorsa ve her yorumcunun cevabının yorumunda gösterilmesi şekilde yapılandırılmışsa, dökümdeki satırlar tıklanabilir hale gelir. Örneğin "5" satırına tıklamak, yorum widget'ını o cevabı veren yorumlara filtreler. Satıra tekrar tıklamak veya yorum widget'ındaki "Yalnızca gösteriliyor" etiketindeki X'e tıklamak, tüm yorumları tekrar gösterir. Bu, yapılandırma gerektirmez: Özet Widget'ı, aynı `tenantId` ve `urlId` değerlerine sahip yorum widget'ını bulur.

Bu, yıldız, başparmak, emoji, ölçek ve tek seçimli sorular için çalışır. Yorum widget'ının yorum kutusunun üzerinde gösterdiği döküm de aynı şekilde tıklanabilir.

Widget'ı, sahip olmadığımız diğer kütüphanelerimizden birinde istiyorsanız, eklememiz gerektiğini bilmemiz için bir destek talebi oluşturun.