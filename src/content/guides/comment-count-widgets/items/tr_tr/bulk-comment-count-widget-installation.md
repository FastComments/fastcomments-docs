Toplu Yorum Sayisi Widget'i, ayni sayfada birden fazla sayfa icin yorum sayilarini verimli bir sekilde gostermek uzere tasarlanmistir. Her yorum sayisi icin ayri API cagrilari yapmak yerine, bu widget en iyi performans icin istekleri gruplar.

## Temel Kurulum

[inline-code-attrs-start title = 'Bulk Comment Count Widget Installation'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-widget-comment-count-bulk.min.js"></script>

<!-- Multiple elements with comment counts -->
<div class="fast-comments-count" data-fast-comments-url-id="page-1"></div>
<div class="fast-comments-count" data-fast-comments-url-id="page-2"></div>
<div class="fast-comments-count" data-fast-comments-url-id="page-3"></div>

<script>
    window.FastCommentsBulkCountConfig = {
        tenantId: 'demo'
    };
</script>
[inline-code-end]

## Nasil Calisir

Toplu widget su sekilde calisir:

1. `fast-comments-count` sinifina sahip ogeler icin sayfayi tarar
2. Her ogeden `data-fast-comments-url-id` ozelligini okur
3. Birden fazla yorum sayisini verimli bir sekilde almak icin API isteklerini gruplar
4. Her ogeyi uygun yorum sayisiyla gunceller

## Yapilandirma Secenekleri

`FastCommentsCommentCountBulk` fonksiyonu asagidaki yapilandirma seceneklerini kabul eder:

- **tenantId** (zorunlu): FastComments kiraci kimliginiz
- **apiHost** (istege bagli): Kendi barindirdiginiz bir ornek kullaniyorsaniz ozel API ana bilgisayari

## Gercek Dunya Ornegi

Iste toplu widget'i bir blog yazisi listesinde nasil kullanabileceginizi gosteren pratik bir ornek:

[inline-code-attrs-start title = 'Blog Post Listing with Comment Counts'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-widget-comment-count-bulk.min.js"></script>

<style>
    .blog-post {
        border: 1px solid #ddd;
        margin: 10px 0;
        padding: 15px;
        border-radius: 5px;
    }
    .post-meta {
        color: #666;
        font-size: 14px;
        margin-top: 10px;
    }
    .comment-count {
        background: #e7f3ff;
        padding: 2px 8px;
        border-radius: 12px;
        font-size: 12px;
        display: inline-block;
    }
</style>

<div class="blog-post">
    <h3>How to Install FastComments</h3>
    <p>Learn how to add FastComments to your website in just a few minutes...</p>
    <div class="post-meta">
        Published: March 15, 2024 |
        <span class="fast-comments-count comment-count" data-fast-comments-url-id="how-to-install-fastcomments"></span>
    </div>
</div>

<div class="blog-post">
    <h3>Advanced FastComments Configuration</h3>
    <p>Dive deep into the advanced configuration options for FastComments...</p>
    <div class="post-meta">
        Published: March 10, 2024 |
        <span class="fast-comments-count comment-count" data-fast-comments-url-id="advanced-fastcomments-config"></span>
    </div>
</div>

<div class="blog-post">
    <h3>FastComments vs Other Solutions</h3>
    <p>See how FastComments compares to other commenting solutions...</p>
    <div class="post-meta">
        Published: March 5, 2024 |
        <span class="fast-comments-count comment-count" data-fast-comments-url-id="fastcomments-comparison"></span>
    </div>
</div>

<script>
    window.FastCommentsBulkCountConfig = {
        tenantId: 'demo'
    };
</script>
[inline-code-end]

## Performans Degerlendirmeleri

Toplu widget asagidakiler araciligiyla performansi otomatik olarak optimize eder:

- **Istek gruplama**: Birden fazla yorum sayisi tek bir API cagrisinda alinir
- **Istek boyutu sinirlari**: URL listesi cok uzun olursa (1.000 karakterden fazla) istekler otomatik olarak bolunur
- **Tekrarlari kaldirma**: Ayni `data-fast-comments-url-id` degerine sahip birden fazla oge ayni sayiyi paylasir

## Ayni URL ID'li Birden Fazla Oge

Sayfada ayni `data-fast-comments-url-id` degerine sahip birden fazla ogeniz olabilir. Hepsi ayni sayiyla guncellenecektir:

[inline-code-attrs-start title = 'Multiple Elements Same URL ID'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-widget-comment-count-bulk.min.js"></script>

<style>
    .count-example {
        margin: 10px 0;
        padding: 10px;
        background: #f9f9f9;
        border-radius: 5px;
    }
</style>

<div class="count-example">
    Header Count: <span class="fast-comments-count" data-fast-comments-url-id="shared-article"></span>
</div>

<div class="count-example">
    Sidebar Count: <span class="fast-comments-count" data-fast-comments-url-id="shared-article"></span>
</div>

<div class="count-example">
    Footer Count: <span class="fast-comments-count" data-fast-comments-url-id="shared-article"></span>
</div>

<script>
    window.FastCommentsBulkCountConfig = {
        tenantId: 'demo'
    };
</script>
[inline-code-end]

## Yerellesirme

Toplu widget, FastComments dil ayarlariniza gore yorum sayilarini otomatik olarak biçimlendirir. Asagidakiler icin uygun metin saglar:

- Sifir yorum
- Bir yorum
- Birden fazla yorum

## Toplu ve Tekli Widget Ne Zaman Kullanilir

**Toplu Widget'i su durumlarda kullanin:**
- Ayni sayfada birden fazla yorum sayiniz var
- Yorum sayilariyla birlikte bir yazi/makale listesi gosteriyorsunuz
- Performans onemli (API cagrilarini azaltir)

**Tekli Widget'i su durumlarda kullanin:**
- Sayfada yalnizca bir yorum sayisina ihtiyaciniz var
- Canli guncellemelere ihtiyaciniz var (tekli widget gercek zamanli guncellemeleri destekler)
- Bireysel widget davranisi uzerinde daha fazla kontrol istiyorsunuz
