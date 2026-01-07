Widget za masovno brojanje komentara dizajniran je za ucinkovito prikazivanje brojeva komentara za vise stranica na istoj stranici. Umjesto pojedinacnih API poziva za svaki broj komentara, ovaj widget grupira zahtjeve za optimalnu izvedbu.

## Osnovna instalacija

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

## Kako funkcionira

Masovni widget funkcionira tako da:

1. Skenira stranicu za elemente s klasom `fast-comments-count`
2. Cita atribut `data-fast-comments-url-id` iz svakog elementa
3. Grupira API zahtjeve za ucinkovito dohvacanje vise brojeva komentara
4. Azurira svaki element s odgovarajucim brojem komentara

## Opcije konfiguracije

Funkcija `FastCommentsCommentCountBulk` prihvaca sljedece opcije konfiguracije:

- **tenantId** (obavezno): Vas FastComments tenant ID
- **apiHost** (opcijski): Prilagodeni API host ako koristite vlastitu instancu

## Primjer iz stvarnog svijeta

Evo prakticnog primjera koji pokazuje kako mozete koristiti masovni widget u popisu blog postova:

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

## Razmatranja o izvedbi

Masovni widget automatski optimizira izvedbu pomocu:

- **Grupiranja zahtjeva**: Vise brojeva komentara dohvaca se u jednom API pozivu
- **Ogranicenja velicine zahtjeva**: Zahtjevi se automatski dijele ako popis URL-ova postane prevelik (preko 1.000 znakova)
- **Deduplikacije**: Vise elemenata s istim `data-fast-comments-url-id` dijeli isti broj

## Vise elemenata s istim URL ID-om

Mozete imati vise elemenata na stranici s istim `data-fast-comments-url-id`. Svi ce biti azurirani s istim brojem:

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

## Lokalizacija

Masovni widget automatski formatira brojeve komentara na temelju vasih FastComments jezicnih postavki. Pruza odgovarajuci tekst za:

- Nula komentara
- Jedan komentar
- Vise komentara

## Kada koristiti masovni widget naspram pojedinacnog

**Koristite masovni widget kada:**
- Imate vise brojeva komentara na istoj stranici
- Prikazujete popis postova/clanaka s brojevima komentara
- Izvedba je vazna (smanjuje API pozive)

**Koristite pojedinacni widget kada:**
- Trebate samo jedan broj komentara na stranici
- Trebate azuriranja uzivo (pojedinacni widget podrzava azuriranja u stvarnom vremenu)
- Zelite vise kontrole nad ponasanjem pojedinacnog widgeta
