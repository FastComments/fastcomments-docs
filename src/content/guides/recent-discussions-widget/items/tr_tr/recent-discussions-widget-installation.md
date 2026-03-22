The Recent Discussions Widget, sitenizde en son yorum etkinliğine sahip sayfaları gösterir. Her öğe sayfa başlığını, son etkinlik tarihini ve toplam yorum sayısını gösterir. Koyu arka planları otomatik olarak algılar ve stilini buna göre ayarlar.

## Basic Installation

[inline-code-attrs-start title = 'Recent Discussions Widget Kurulumu'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/widget-recent-discussions-v2.min.js"></script>
<div id="fastcomments-widget-recent-discussions"></div>
<script>
    FastCommentsRecentDiscussionsV2(document.getElementById('fastcomments-widget-recent-discussions'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

## Configuration Options

`FastCommentsRecentDiscussionsV2` fonksiyonu aşağıdaki yapılandırma seçeneklerini kabul eder:

- **tenantId** (required): FastComments kiracı kimliğiniz
- **count** (optional): Gösterilecek sayfa sayısı. Varsayılan `20`, maksimum `100`
- **hasDarkBackground** (optional): Koyu mod stilini zorlar. Ayarlanmazsa sayfa arka planından otomatik olarak algılanır

## Advanced Examples

### Custom Count

[inline-code-attrs-start title = 'Recent Discussions — Özel Sayı'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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

### Force Dark Mode

[inline-code-attrs-start title = 'Recent Discussions — Koyu Mod'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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

---