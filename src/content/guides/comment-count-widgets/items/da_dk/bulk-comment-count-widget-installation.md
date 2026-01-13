Bulk Comment Count Widget er designet til effektivt at vise kommentartællinger for flere sider på samme side. I stedet for at foretage individuelle API-kald for hver kommentartælling, samler denne widget anmodninger for optimal ydeevne.

## Grundlæggende installation

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

## Sådan virker det

Bulk-widgetten fungerer ved at:

1. Scanne siden for elementer med klassen `fast-comments-count`
2. Læse `data-fast-comments-url-id` attributten fra hvert element
3. Samle API-anmodninger for effektivt at hente flere kommentartællinger
4. Opdatere hvert element med den relevante kommentartælling

## Konfigurationsmuligheder

Funktionen `FastCommentsCommentCountBulk` accepterer følgende konfigurationsmuligheder:

- **tenantId** (påkrævet): Dit FastComments tenant-ID
- **apiHost** (valgfrit): Brugerdefineret API-vært, hvis du bruger en selv-hostet instans

## Eksempel fra den virkelige verden

Her er et praktisk eksempel, der viser, hvordan du kan bruge bulk-widgetten i en blogindlægsliste:

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

## Ydeevneovervejelser

Bulk-widgetten optimerer automatisk ydeevnen ved at:

- **Samle anmodninger**: Flere kommentartællinger hentes i et enkelt API-kald
- **Anmodningsstørrelsesbegrænsninger**: Anmodninger opdeles automatisk, hvis URL-listen bliver for stor (over 1.000 tegn)
- **Deduplikering**: Flere elementer med samme `data-fast-comments-url-id` deler samme tælling

## Flere elementer med samme URL-ID

Du kan have flere elementer på siden med samme `data-fast-comments-url-id`. De vil alle blive opdateret med samme tælling:

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

## Lokalisering

Bulk-widgetten formaterer automatisk kommentartællinger baseret på dine FastComments sprogindstillinger. Den giver passende tekst til:

- Nul kommentarer
- En kommentar
- Flere kommentarer

## Hvornår skal man bruge Bulk vs Single Widget

**Brug Bulk Widget når:**
- Du har flere kommentartællinger på samme side
- Du viser en liste over indlæg/artikler med kommentartællinger
- Ydeevne er vigtig (reducerer API-kald)

**Brug Single Widget når:**
- Du kun har brug for en kommentartælling på siden
- Du har brug for live-opdateringer (single-widgetten understøtter realtidsopdateringer)
- Du ønsker mere kontrol over individuel widget-adfærd
