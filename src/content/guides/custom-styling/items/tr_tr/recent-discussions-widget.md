Recent Discussions Widget, en son yorum etkinliğine göre sıralanmış bir sayfa listesi gösterir. Başlık, son etkinlik tarihleri, simgeli yorum sayıları ve otomatik karanlık mod algılama içerir.

## Temel Kurulum

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

## Yapılandırma Seçenekleri

- **tenantId** (gerekli): FastComments kiracı kimliğiniz
- **count** (opsiyonel): Gösterilecek sayfa sayısı. Varsayılan `20`, maksimum `100`
- **hasDarkBackground** (opsiyonel): Karanlık mod stilini zorlar. Ayarlanmazsa sayfa arka planından otomatik algılanır

## Widget Yapısı

Widget aşağıdaki HTML yapısıyla render edilir:

[inline-code-attrs-start title = 'Recent Discussions Widget HTML Yapısı'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<div class="fc-rd2">
    <div class="fc-rd2-heading">Recent Discussions</div>
    <div class="fc-rd2-list">
        <div class="fc-rd2-item">
            <div class="fc-rd2-detail">
                <a class="fc-rd2-title" href="...">Page Title</a>
                <span class="fc-rd2-activity">Last activity Mar 21, 2026</span>
            </div>
            <div class="fc-rd2-count">42</div>
        </div>
    </div>
</div>
[inline-code-end]

## Varsayılan CSS Referansı

[inline-code-attrs-start title = 'Recent Discussions Widget Varsayılan CSS'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
.fc-rd2 {
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Oxygen, Ubuntu, Cantarell, "Helvetica Neue", sans-serif;
    text-align: left;
    line-height: 1.5;
    color: #1a1a1a;
    border: 1px solid #e0e0e0;
    border-radius: 12px;
    padding: 20px;
    background: #fff;
}
.fc-rd2-heading { font-size: 16px; font-weight: 700; margin-bottom: 14px; padding-bottom: 12px; border-bottom: 1px solid #eee; }
.fc-rd2-item { display: flex; align-items: center; gap: 12px; padding: 10px 0; border-bottom: 1px solid #f0f0f0; }
.fc-rd2-item:last-child { border-bottom: none; }
.fc-rd2-title { font-size: 13px; font-weight: 500; color: #1a1a1a; text-decoration: none; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
.fc-rd2-activity { font-size: 11px; color: #999; }
.fc-rd2-count { font-size: 12px; font-weight: 600; color: #666; }
[inline-code-end]

## Özelleştirme Örnekleri

### Konteyner Kenarlığını Kaldırma

[inline-code-attrs-start title = 'Konteyner Kenarlığını Kaldır'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
.fc-rd2 {
    border: none !important;
    box-shadow: none !important;
}
[inline-code-end]

### Özel Bağlantı Rengi

[inline-code-attrs-start title = 'Özel Bağlantı Rengi'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
a.fc-rd2-title:hover {
    color: #e63946 !important;
}
[inline-code-end]

---