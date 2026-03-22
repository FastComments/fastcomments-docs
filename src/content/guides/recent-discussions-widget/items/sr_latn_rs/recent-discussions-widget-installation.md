The Recent Discussions Widget prikazuje stranice na vašem sajtu koje imaju najnoviju aktivnost komentara. Svaki unos prikazuje naslov stranice, datum poslednje aktivnosti i ukupan broj komentara. Automatski detektuje tamne pozadine i prilagođava svoj stil u skladu s tim.

## Osnovna instalacija

[inline-code-attrs-start title = 'Instalacija vidžeta Nedavne diskusije'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/widget-recent-discussions-v2.min.js"></script>
<div id="fastcomments-widget-recent-discussions"></div>
<script>
    FastCommentsRecentDiscussionsV2(document.getElementById('fastcomments-widget-recent-discussions'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

## Opcije konfiguracije

The `FastCommentsRecentDiscussionsV2` function accepts the following configuration options:

- **tenantId** (required): ID vašeg FastComments naloga
- **count** (optional): Number of pages to show. Default is `20`, max `100`
- **hasDarkBackground** (optional): Prisili tamni režim stilizacije. Ako nije postavljeno, automatski se detektuje iz pozadine stranice

## Napredni primeri

### Prilagođeni broj

[inline-code-attrs-start title = 'Nedavne diskusije sa prilagođenim brojem'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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

### Prisili tamni režim

[inline-code-attrs-start title = 'Nedavne diskusije u tamnom režimu'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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