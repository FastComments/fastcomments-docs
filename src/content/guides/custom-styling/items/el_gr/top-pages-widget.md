Το widget Most Discussed Pages εμφανίζει μια ταξινομημένη λίστα με τις σελίδες σας που έχουν τα περισσότερα σχόλια. Περιλαμβάνει μια κεφαλίδα, αριθμημένες θέσεις κατάταξης, μετρήσεις σχολίων με εικονίδια, ημερομηνίες τελευταίας δραστηριότητας και αυτόματη ανίχνευση dark mode.

## Βασική Εγκατάσταση

[inline-code-attrs-start title = 'Εγκατάσταση widget Πιο Σχολιασμένων Σελίδων'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/widget-top-pages-v2.min.js"></script>
<div id="fastcomments-widget-top-pages"></div>
<script>
    FastCommentsTopPagesV2(document.getElementById('fastcomments-widget-top-pages'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

## Επιλογές Διαμόρφωσης

- **tenantId** (απαιτείται): Το tenant ID του FastComments
- **hasDarkBackground** (προαιρετικό): Επιβάλλει το στυλ dark mode. Αν δεν οριστεί, ανιχνεύεται αυτόματα από το φόντο της σελίδας

## Δομή του Widget

Το widget αποδίδεται με την ακόλουθη δομή HTML:

[inline-code-attrs-start title = 'Δομή HTML του widget Πιο Σχολιασμένων Σελίδων'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<div class="fc-tp2">
    <div class="fc-tp2-heading">Most Discussed Pages</div>
    <div class="fc-tp2-list">
        <div class="fc-tp2-item">
            <div class="fc-tp2-rank">1</div>
            <div class="fc-tp2-detail">
                <a class="fc-tp2-title" href="...">Page Title</a>
                <span class="fc-tp2-activity">Last activity Mar 21, 2026</span>
            </div>
            <div class="fc-tp2-count">42</div>
        </div>
    </div>
</div>
[inline-code-end]

## Αναφορά Προεπιλεγμένου CSS

[inline-code-attrs-start title = 'Προεπιλεγμένο CSS του widget Πιο Σχολιασμένων Σελίδων'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
.fc-tp2 {
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Oxygen, Ubuntu, Cantarell, "Helvetica Neue", sans-serif;
    text-align: left;
    line-height: 1.5;
    color: #1a1a1a;
    border: 1px solid #e0e0e0;
    border-radius: 12px;
    padding: 20px;
    background: #fff;
}
.fc-tp2-heading { font-size: 16px; font-weight: 700; margin-bottom: 14px; padding-bottom: 12px; border-bottom: 1px solid #eee; }
.fc-tp2-item { display: flex; align-items: center; gap: 12px; padding: 10px 0; border-bottom: 1px solid #f0f0f0; }
.fc-tp2-item:last-child { border-bottom: none; }
.fc-tp2-rank { width: 26px; height: 26px; display: flex; align-items: center; justify-content: center; border-radius: 50%; font-size: 11px; font-weight: 700; background: #f0f0f0; color: #888; }
.fc-tp2-title { font-size: 13px; font-weight: 500; color: #1a1a1a; text-decoration: none; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
.fc-tp2-activity { font-size: 11px; color: #999; }
.fc-tp2-count { font-size: 12px; font-weight: 600; color: #666; }
[inline-code-end]

## Παραδείγματα Προσαρμογής

### Αφαίρεση ετικετών κατάταξης

[inline-code-attrs-start title = 'Αφαίρεση ετικετών κατάταξης'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
.fc-tp2-rank {
    display: none !important;
}
[inline-code-end]

### Αφαίρεση περιγράμματος κοντέινερ

[inline-code-attrs-start title = 'Αφαίρεση περιγράμματος κοντέινερ'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
.fc-tp2 {
    border: none !important;
    box-shadow: none !important;
}
[inline-code-end]

---