Pripomoček za množično štetje komentarjev je zasnovan za učinkovito prikazovanje števila komentarjev za več strani na isti strani. Namesto posameznih API klicev za vsako število komentarjev, ta pripomoček združuje zahteve za optimalno zmogljivost.

## Osnovna namestitev

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

## Kako deluje

Množični pripomoček deluje tako, da:

1. Pregleda stran za elemente z razredom `fast-comments-count`
2. Prebere atribut `data-fast-comments-url-id` iz vsakega elementa
3. Združi API zahteve za učinkovito pridobivanje več števil komentarjev
4. Posodobi vsak element z ustreznim številom komentarjev

## Možnosti konfiguracije

Funkcija `FastCommentsCommentCountBulk` sprejema naslednje možnosti konfiguracije:

- **tenantId** (obvezno): Vaš FastComments ID najemnika
- **apiHost** (neobvezno): Prilagojen API gostitelj, če uporabljate lastno gostovano instanco

## Primer iz resničnega sveta

Tukaj je praktičen primer, ki prikazuje, kako lahko uporabite množični pripomoček v seznamu objav na blogu:

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

## Premisleki o zmogljivosti

Množični pripomoček samodejno optimizira zmogljivost z:

- **Združevanjem zahtev**: Več števil komentarjev se pridobi z enim samim API klicem
- **Omejitvami velikosti zahtev**: Zahteve se samodejno razdelijo, če seznam URL-jev postane predolg (več kot 1.000 znakov)
- **Deduplikacijo**: Več elementov z istim `data-fast-comments-url-id` si deli isto število

## Več elementov z istim URL ID

Na strani lahko imate več elementov z istim `data-fast-comments-url-id`. Vsi bodo posodobljeni z istim številom:

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

Množični pripomoček samodejno oblikuje števila komentarjev glede na vaše jezikovne nastavitve FastComments. Zagotavlja ustrezno besedilo za:

- Nič komentarjev
- En komentar
- Več komentarjev

## Kdaj uporabiti množični ali posamični pripomoček

**Uporabite množični pripomoček, ko:**
- Imate več števil komentarjev na isti strani
- Prikazujete seznam objav/člankov s števili komentarjev
- Je zmogljivost pomembna (zmanjšuje API klice)

**Uporabite posamični pripomoček, ko:**
- Potrebujete le eno število komentarjev na strani
- Potrebujete posodobitve v živo (posamični pripomoček podpira posodobitve v realnem času)
- Želite več nadzora nad vedenjem posameznega pripomočka
