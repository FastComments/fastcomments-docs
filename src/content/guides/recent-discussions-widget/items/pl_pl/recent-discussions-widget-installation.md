The Recent Discussions Widget pokazuje strony na Twojej witrynie, które mają najnowszą aktywność komentarzy. Każdy wpis wyświetla tytuł strony, datę ostatniej aktywności oraz łączną liczbę komentarzy. Widżet automatycznie wykrywa ciemne tła i odpowiednio dostosowuje swój styl.

## Podstawowa instalacja

[inline-code-attrs-start title = 'Instalacja widżetu "Najnowsze dyskusje"'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/widget-recent-discussions-v2.min.js"></script>
<div id="fastcomments-widget-recent-discussions"></div>
<script>
    FastCommentsRecentDiscussionsV2(document.getElementById('fastcomments-widget-recent-discussions'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

## Opcje konfiguracji

Funkcja `FastCommentsRecentDiscussionsV2` przyjmuje następujące opcje konfiguracyjne:

- **tenantId** (wymagane): Twój identyfikator najemcy FastComments
- **count** (opcjonalne): Liczba stron do wyświetlenia. Domyślnie `20`, maks. `100`
- **hasDarkBackground** (opcjonalne): Wymusza stylizację w trybie ciemnym. Jeśli nie ustawiono, wykrywane automatycznie na podstawie tła strony

## Zaawansowane przykłady

### Niestandardowa liczba

[inline-code-attrs-start title = 'Najnowsze dyskusje z niestandardową liczbą'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/widget-recent-discussions-v2.min.js"></script>
<div id="fastcomments-widget-recent-discussions"></div>
<script>
    FastCommentsRecentDiscussionsV2(document.getElementById('fastcomments-widget-recent-discussions'), {
        tenantId: 'demo',
        count: 5
    });
</script>
[inline-code-end]

### Wymuś tryb ciemny

[inline-code-attrs-start title = 'Najnowsze dyskusje w trybie ciemnym'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/widget-recent-discussions-v2.min.js"></script>
<div id="fastcomments-widget-recent-discussions"></div>
<script>
    FastCommentsRecentDiscussionsV2(document.getElementById('fastcomments-widget-recent-discussions'), {
        tenantId: 'demo',
        hasDarkBackground: true
    });
</script>
[inline-code-end]

---