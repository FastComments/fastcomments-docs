Das Bulk Comment Count Widget ist dafuer konzipiert, Kommentarzaehlungen fuer mehrere Seiten effizient auf derselben Seite anzuzeigen. Anstatt individuelle API-Aufrufe fuer jede Kommentarzaehlung zu machen, buendelt dieses Widget Anfragen fuer optimale Leistung.

## Grundlegende Installation

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

## Funktionsweise

Das Bulk-Widget funktioniert durch:

1. Scannen der Seite nach Elementen mit der Klasse `fast-comments-count`
2. Lesen des `data-fast-comments-url-id` Attributs von jedem Element
3. Buendeln von API-Anfragen, um mehrere Kommentarzaehlungen effizient abzurufen
4. Aktualisieren jedes Elements mit der entsprechenden Kommentarzaehlung

## Konfigurationsoptionen

Die Funktion `FastCommentsCommentCountBulk` akzeptiert die folgenden Konfigurationsoptionen:

- **tenantId** (erforderlich): Ihre FastComments Tenant-ID
- **apiHost** (optional): Benutzerdefinierter API-Host, wenn Sie eine selbst gehostete Instanz verwenden

## Praxisbeispiel

Hier ist ein praktisches Beispiel, das zeigt, wie Sie das Bulk-Widget in einer Blog-Beitragsliste verwenden koennten:

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

## Leistungsueberlegungen

Das Bulk-Widget optimiert automatisch die Leistung durch:

- **Buendelung von Anfragen**: Mehrere Kommentarzaehlungen werden in einem einzigen API-Aufruf abgerufen
- **Anforderungsgroessenbeschraenkungen**: Anfragen werden automatisch aufgeteilt, wenn die URL-Liste zu gross wird (ueber 1.000 Zeichen)
- **Deduplizierung**: Mehrere Elemente mit derselben `data-fast-comments-url-id` teilen dieselbe Zaehlung

## Mehrere Elemente mit derselben URL-ID

Sie koennen mehrere Elemente auf der Seite mit derselben `data-fast-comments-url-id` haben. Sie werden alle mit derselben Zaehlung aktualisiert:

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

## Lokalisierung

Das Bulk-Widget formatiert Kommentarzaehlungen automatisch basierend auf Ihren FastComments-Spracheinstellungen. Es bietet passenden Text fuer:

- Null Kommentare
- Einen Kommentar
- Mehrere Kommentare

## Wann Bulk vs Single Widget verwenden

**Verwenden Sie das Bulk Widget wenn:**
- Sie mehrere Kommentarzaehlungen auf derselben Seite haben
- Sie eine Liste von Beitraegen/Artikeln mit Kommentarzaehlungen anzeigen
- Leistung wichtig ist (reduziert API-Aufrufe)

**Verwenden Sie das Single Widget wenn:**
- Sie nur eine Kommentarzaehlung auf der Seite benoetigen
- Sie Live-Updates benoetigen (das Single-Widget unterstuetzt Echtzeit-Updates)
- Sie mehr Kontrolle ueber das individuelle Widget-Verhalten wuenschen
