Widżet Ostatnich Komentarzy wyświetla listę najnowszych komentarzy z całej witryny lub z konkretnej strony. Zawiera nagłówek, zaokrąglone awatary, podglądy komentarzy, klikalne daty prowadzące bezpośrednio do komentarza oraz automatyczne wykrywanie trybu ciemnego.

## Podstawowa instalacja

[inline-code-attrs-start title = 'Instalacja widżetu Ostatnich Komentarzy'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/widget-recent-comments-v2.min.js"></script>
<div id="fastcomments-widget-recent-comments"></div>
<script>
    FastCommentsRecentCommentsV2(document.getElementById('fastcomments-widget-recent-comments'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

## Opcje konfiguracji

- **tenantId** (wymagane): Identyfikator tenant FastComments
- **urlId** (opcjonalne): Filtr ograniczający do jednej strony. Pozostaw null dla wszystkich stron
- **count** (opcjonalne): Liczba wyświetlanych komentarzy. Domyślnie `10`
- **hasDarkBackground** (opcjonalne): Wymusza stylizację trybu ciemnego. Automatycznie wykrywane na podstawie tła strony, jeśli nie ustawiono

## Struktura widżetu

Widżet renderuje się z następującą strukturą HTML:

[inline-code-attrs-start title = 'Struktura HTML widżetu Ostatnich Komentarzy'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<div class="fc-rc2">
    <div class="fc-rc2-heading">Recent Comments</div>
    <div class="fc-rc2-list">
        <div class="fc-rc2-card">
            <div class="fc-rc2-header">
                <img class="fc-rc2-avatar" src="..." alt="Avatar" />
                <div class="fc-rc2-meta">
                    <span class="fc-rc2-name">Username</span>
                    <a class="fc-rc2-date" href="...">2 hours ago</a>
                </div>
            </div>
            <div class="fc-rc2-body">Comment preview...</div>
            <a class="fc-rc2-page-link" href="...">Page Title</a>
        </div>
    </div>
</div>
[inline-code-end]

## Referencja domyślnego CSS

[inline-code-attrs-start title = 'Domyślny CSS widżetu Ostatnich Komentarzy'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
.fc-rc2 {
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Oxygen, Ubuntu, Cantarell, "Helvetica Neue", sans-serif;
    text-align: left;
    line-height: 1.5;
    color: #1a1a1a;
    border: 1px solid #e0e0e0;
    border-radius: 12px;
    padding: 20px;
    background: #fff;
}
.fc-rc2-heading { font-size: 16px; font-weight: 700; margin-bottom: 14px; padding-bottom: 12px; border-bottom: 1px solid #eee; }
.fc-rc2-card { padding: 14px 0; border-bottom: 1px solid #f0f0f0; }
.fc-rc2-card:last-child { border-bottom: none; }
.fc-rc2-header { display: flex; align-items: center; gap: 10px; margin-bottom: 8px; }
.fc-rc2-avatar { width: 32px; height: 32px; border-radius: 50%; object-fit: cover; }
.fc-rc2-name { font-size: 13px; font-weight: 600; }
.fc-rc2-date { font-size: 11.5px; color: #999; text-decoration: none; }
.fc-rc2-body { font-size: 14px; line-height: 1.55; color: #444; display: -webkit-box; -webkit-line-clamp: 2; -webkit-box-orient: vertical; overflow: hidden; }
.fc-rc2-page-link { display: inline-block; margin-top: 6px; font-size: 12px; color: #777; text-decoration: none; }
.fc-rc2-page-link:hover { color: #0066cc; text-decoration: underline; }
[inline-code-end]

## Przykłady dostosowania

### Zmiana rozmiaru awatara

[inline-code-attrs-start title = 'Niestandardowy rozmiar awatara'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
.fc-rc2-avatar {
    width: 40px !important;
    height: 40px !important;
}
[inline-code-end]

### Wyświetlanie większej liczby linii tekstu komentarza

[inline-code-attrs-start title = 'Wyświetlanie większej liczby linii komentarza'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
.fc-rc2-body {
    -webkit-line-clamp: 5 !important;
}
[inline-code-end]

### Usunięcie obramowania kontenera

[inline-code-attrs-start title = 'Usunięcie obramowania kontenera'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
.fc-rc2 {
    border: none !important;
    box-shadow: none !important;
}
[inline-code-end]