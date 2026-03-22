The Most Discussed Pages Widget, en çok yorumlanan sayfalarınızın sıralı bir listesini görüntüler. Başlık, numaralandırılmış sıralar, simgeli yorum sayıları, son etkinlik tarihleri ve otomatik karanlık mod algılama içerir.

## Temel Kurulum

[inline-code-attrs-start title = 'Most Discussed Pages Widget Kurulumu'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/widget-top-pages-v2.min.js"></script>
<div id="fastcomments-widget-top-pages"></div>
<script>
    FastCommentsTopPagesV2(document.getElementById('fastcomments-widget-top-pages'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

## Yapılandırma Seçenekleri

- **tenantId** (zorunlu): FastComments tenant ID'niz
- **hasDarkBackground** (isteğe bağlı): Karanlık mod stilini zorla. Ayarlanmazsa sayfa arka planından otomatik algılanır

## Widget Yapısı

Widget aşağıdaki HTML yapısı ile render edilir:

[inline-code-attrs-start title = 'Most Discussed Pages Widget HTML Yapısı'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<div class="fc-tp2">
    <div class="fc-tp2-heading">Most Discussed Pages</div>
    <div class="fc-tp2-list">
        <div class="fc-tp2-item">
            <div class="fc-tp2-rank">1</div>
            <div class="fc-tp2-detail">
                <a class="fc-tp2-title" href="...">Page Title</a>
                <span class="fc-tp2-activity">Last activity Mar 21, 2026</span>
            </div>
            <div class="fc-tp2-count">42</div>
        </div>
    </div>
</div>
[inline-code-end]

## Varsayılan CSS Referansı

[inline-code-attrs-start title = 'Most Discussed Pages Widget Varsayılan CSS'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
.fc-tp2 {
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Oxygen, Ubuntu, Cantarell, "Helvetica Neue", sans-serif;
    text-align: left;
    line-height: 1.5;
    color: #1a1a1a;
    border: 1px solid #e0e0e0;
    border-radius: 12px;
    padding: 20px;
    background: #fff;
}
.fc-tp2-heading { font-size: 16px; font-weight: 700; margin-bottom: 14px; padding-bottom: 12px; border-bottom: 1px solid #eee; }
.fc-tp2-item { display: flex; align-items: center; gap: 12px; padding: 10px 0; border-bottom: 1px solid #f0f0f0; }
.fc-tp2-item:last-child { border-bottom: none; }
.fc-tp2-rank { width: 26px; height: 26px; display: flex; align-items: center; justify-content: center; border-radius: 50%; font-size: 11px; font-weight: 700; background: #f0f0f0; color: #888; }
.fc-tp2-title { font-size: 13px; font-weight: 500; color: #1a1a1a; text-decoration: none; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
.fc-tp2-activity { font-size: 11px; color: #999; }
.fc-tp2-count { font-size: 12px; font-weight: 600; color: #666; }
[inline-code-end]

## Özelleştirme Örnekleri

### Sıralama Rozetlerini Kaldır

[inline-code-attrs-start title = 'Sıralama Rozetlerini Kaldır'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
.fc-tp2-rank {
    display: none !important;
}
[inline-code-end]

### Konteyner Kenarlığını Kaldır

[inline-code-attrs-start title = 'Konteyner Kenarlığını Kaldır'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
.fc-tp2 {
    border: none !important;
    box-shadow: none !important;
}
[inline-code-end]

---