The Recent Discussions Widget prikazuje stranice na vašoj web-lokaciji koje imaju najnoviju aktivnost komentara. Svaki unos prikazuje naslov stranice, datum posljednje aktivnosti i ukupan broj komentara. Automatski prepoznaje tamne pozadine i prilagođava svoj izgled u skladu s tim.

## Osnovna instalacija

[inline-code-attrs-start title = 'Instalacija widgeta Recent Discussions'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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

Funkcija `FastCommentsRecentDiscussionsV2` prihvaća sljedeće opcije konfiguracije:

- **tenantId** (obavezno): Vaš FastComments tenant ID
- **count** (neobavezno): Broj stranica za prikaz. Zadano je `20`, maksimum `100`
- **hasDarkBackground** (neobavezno): Prisilite tamni način prikaza. Ako nije postavljeno, automatski se otkriva prema pozadini stranice

## Napredni primjeri

### Prilagođeni broj

[inline-code-attrs-start title = 'Recent Discussions s prilagođenim brojem'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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

### Prisili tamni način

[inline-code-attrs-start title = 'Recent Discussions s tamnim načinom rada'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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