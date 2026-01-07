De Bulk Comment Count Widget is ontworpen voor het efficient weergeven van commentaartellingen voor meerdere pagina's op dezelfde pagina. In plaats van individuele API-aanroepen te maken voor elke commentaartelling, bundelt deze widget verzoeken voor optimale prestaties.

## Basis Installatie

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

## Hoe Het Werkt

De bulk widget werkt door:

1. De pagina te scannen naar elementen met de klasse `fast-comments-count`
2. Het `data-fast-comments-url-id` attribuut van elk element te lezen
3. API-verzoeken te bundelen om meerdere commentaartellingen efficient op te halen
4. Elk element bij te werken met de juiste commentaartelling

## Configuratie Opties

De `FastCommentsCommentCountBulk` functie accepteert de volgende configuratieopties:

- **tenantId** (vereist): Uw FastComments tenant ID
- **apiHost** (optioneel): Aangepaste API-host als u een zelf-gehoste instantie gebruikt

## Praktijkvoorbeeld

Hier is een praktisch voorbeeld dat laat zien hoe u de bulk widget kunt gebruiken in een blogpost-overzicht:

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

## Prestatie Overwegingen

De bulk widget optimaliseert automatisch de prestaties door:

- **Verzoeken bundelen**: Meerdere commentaartellingen worden opgehaald in een enkele API-aanroep
- **Verzoekgrootte limieten**: Verzoeken worden automatisch opgesplitst als de URL-lijst te groot wordt (meer dan 1.000 tekens)
- **Deduplicatie**: Meerdere elementen met dezelfde `data-fast-comments-url-id` delen dezelfde telling

## Meerdere Elementen met Dezelfde URL ID

U kunt meerdere elementen op de pagina hebben met dezelfde `data-fast-comments-url-id`. Ze worden allemaal bijgewerkt met dezelfde telling:

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

## Lokalisatie

De bulk widget formatteert automatisch commentaartellingen op basis van uw FastComments taalinstellingen. Het biedt passende tekst voor:

- Nul reacties
- Een reactie
- Meerdere reacties

## Wanneer Bulk vs Single Widget Gebruiken

**Gebruik de Bulk Widget wanneer:**
- U meerdere commentaartellingen op dezelfde pagina heeft
- U een lijst van berichten/artikelen met commentaartellingen weergeeft
- Prestaties belangrijk zijn (vermindert API-aanroepen)

**Gebruik de Single Widget wanneer:**
- U slechts een commentaartelling op de pagina nodig heeft
- U live updates nodig heeft (de single widget ondersteunt realtime updates)
- U meer controle wilt over het gedrag van individuele widgets
