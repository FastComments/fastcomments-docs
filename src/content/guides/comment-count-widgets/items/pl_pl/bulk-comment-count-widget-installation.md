Widget Masowego Liczenia Komentarzy jest zaprojektowany do wydajnego wyswietlania liczby komentarzy dla wielu stron na tej samej stronie. Zamiast wykonywac indywidualne wywolania API dla kazdej liczby komentarzy, ten widget laczy zadania dla optymalnej wydajnosci.

## Podstawowa Instalacja

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

## Jak To Dziala

Widget masowy dziala poprzez:

1. Skanowanie strony w poszukiwaniu elementow z klasa `fast-comments-count`
2. Odczytywanie atrybutu `data-fast-comments-url-id` z kazdego elementu
3. Laczenie zadan API w celu wydajnego pobierania wielu liczb komentarzy
4. Aktualizowanie kazdego elementu odpowiednia liczba komentarzy

## Opcje Konfiguracji

Funkcja `FastCommentsCommentCountBulk` akceptuje nastepujace opcje konfiguracji:

- **tenantId** (wymagane): Twoj identyfikator najemcy FastComments
- **apiHost** (opcjonalne): Niestandardowy host API, jesli uzywasz instancji samodzielnie hostowanej

## Przyklad z Rzeczywistego Swiata

Oto praktyczny przyklad pokazujacy, jak mozesz uzyc widgetu masowego w liscie wpisow na blogu:

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

## Kwestie Wydajnosci

Widget masowy automatycznie optymalizuje wydajnosc poprzez:

- **Laczenie zadan**: Wiele liczb komentarzy jest pobieranych w jednym wywolaniu API
- **Limity rozmiaru zadan**: Zadania sa automatycznie dzielone, jesli lista URL staje sie zbyt dluga (ponad 1000 znakow)
- **Deduplikacja**: Wiele elementow z tym samym `data-fast-comments-url-id` uzywa tej samej liczby

## Wiele Elementow z Tym Samym URL ID

Mozesz miec wiele elementow na stronie z tym samym `data-fast-comments-url-id`. Wszystkie zostana zaktualizowane ta sama liczba:

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

## Lokalizacja

Widget masowy automatycznie formatuje liczby komentarzy na podstawie ustawien jezyka FastComments. Zapewnia odpowiedni tekst dla:

- Zero komentarzy
- Jeden komentarz
- Wiele komentarzy

## Kiedy Uzywac Widgetu Masowego vs Pojedynczego

**Uzyj widgetu masowego gdy:**
- Masz wiele liczb komentarzy na tej samej stronie
- Wyswietlasz liste wpisow/artykulow z liczbami komentarzy
- Wydajnosc jest wazna (zmniejsza wywolania API)

**Uzyj widgetu pojedynczego gdy:**
- Potrzebujesz tylko jednej liczby komentarzy na stronie
- Potrzebujesz aktualizacji na zywo (widget pojedynczy obsluguje aktualizacje w czasie rzeczywistym)
- Chcesz wiecej kontroli nad zachowaniem pojedynczego widgetu
