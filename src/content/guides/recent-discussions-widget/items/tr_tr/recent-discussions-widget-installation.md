Son Tartışmalar Widget'ı, sitenizde en son yorum etkinliğine sahip sayfaları gösterir. Her giriş, sayfa başlığını, son etkinlik tarihini ve toplam yorum sayısını görüntüler. Koyu arka planları otomatik olarak algılar ve stilini buna göre ayarlar.

## Temel Kurulum

[inline-code-attrs-start title = 'Son Tartışmalar Widget Kurulumu'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/widget-recent-discussions-v2.min.js"></script>
<div id="fastcomments-widget-recent-discussions"></div>
<script>
    FastCommentsRecentDiscussionsV2(document.getElementById('fastcomments-widget-recent-discussions'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

## Yapılandırma Seçenekleri

`FastCommentsRecentDiscussionsV2` fonksiyonu aşağıdaki yapılandırma seçeneklerini kabul eder:

- **tenantId** (zorunlu): FastComments tenant ID'niz
- **count** (isteğe bağlı): Gösterilecek sayfa sayısı. Varsayılan `20`, maksimum `100`
- **hasDarkBackground** (isteğe bağlı): Koyu mod stilini zorla. Ayarlanmazsa sayfa arka planından otomatik algılanır

## Gelişmiş Örnekler

### Özel Sayı

[inline-code-attrs-start title = 'Özel Sayılı Son Tartışmalar'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/widget-recent-discussions-v2.min.js"></script>
<div id="fastcomments-widget-recent-discussions"></div>
<script>
    FastCommentsRecentDiscussionsV2(document.getElementById('fastcomments-widget-recent-discussions'), {
        tenantId: 'demo',
        count: 5
    });
</script>
[inline-code-end]

### Koyu Modu Zorla

[inline-code-attrs-start title = 'Koyu Modlu Son Tartışmalar'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/widget-recent-discussions-v2.min.js"></script>
<div id="fastcomments-widget-recent-discussions"></div>
<script>
    FastCommentsRecentDiscussionsV2(document.getElementById('fastcomments-widget-recent-discussions'), {
        tenantId: 'demo',
        hasDarkBackground: true
    });
</script>
[inline-code-end]