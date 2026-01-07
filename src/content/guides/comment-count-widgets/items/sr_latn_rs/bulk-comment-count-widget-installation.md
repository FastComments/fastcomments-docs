Vidžet za masovno brojanje komentara je dizajniran za efikasno prikazivanje broja komentara za više stranica na istoj stranici. Umesto pojedinačnih API poziva za svaki broj komentara, ovaj vidžet grupiše zahteve za optimalne performanse.

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

## Kako funkcioniše

Masovni vidžet funkcioniše tako što:

1. Skenira stranicu za elemente sa klasom `fast-comments-count`
2. Čita atribut `data-fast-comments-url-id` iz svakog elementa
3. Grupiše API zahteve za efikasno preuzimanje više brojeva komentara
4. Ažurira svaki element odgovarajućim brojem komentara

## Opcije konfiguracije

Funkcija `FastCommentsCommentCountBulk` prihvata sledeće opcije konfiguracije:

- **tenantId** (obavezno): Vaš FastComments ID zakupca
- **apiHost** (opciono): Prilagođeni API host ako koristite sopstvenu instancu

## Primer iz stvarnog sveta

Evo praktičnog primera koji pokazuje kako možete koristiti masovni vidžet u listi blog postova:

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

## Razmatranja performansi

Masovni vidžet automatski optimizuje performanse putem:

- **Grupisanja zahteva**: Više brojeva komentara se preuzima u jednom API pozivu
- **Ograničenja veličine zahteva**: Zahtevi se automatski dele ako lista URL-ova postane predugačka (preko 1.000 karaktera)
- **Deduplikacije**: Više elemenata sa istim `data-fast-comments-url-id` dele isti broj

## Više elemenata sa istim URL ID

Možete imati više elemenata na stranici sa istim `data-fast-comments-url-id`. Svi će biti ažurirani istim brojem:

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

Masovni vidžet automatski formatira brojeve komentara na osnovu vaših FastComments jezičkih podešavanja. Pruža odgovarajući tekst za:

- Nula komentara
- Jedan komentar
- Više komentara

## Kada koristiti masovni naspram pojedinačnog vidžeta

**Koristite masovni vidžet kada:**
- Imate više brojeva komentara na istoj stranici
- Prikazujete listu postova/članaka sa brojevima komentara
- Performanse su važne (smanjuje API pozive)

**Koristite pojedinačni vidžet kada:**
- Potreban vam je samo jedan broj komentara na stranici
- Potrebna su vam ažuriranja uživo (pojedinačni vidžet podržava ažuriranja u realnom vremenu)
- Želite više kontrole nad ponašanjem pojedinačnog vidžeta
