Widget za nedavne diskusije prikazuje stranice na vašem sajtu koje imaju najnoviju aktivnost komentara. Svaki unos prikazuje naslov stranice, datum poslednje aktivnosti i ukupan broj komentara. Automatski detektuje tamne pozadine i prilagođava svoj stil u skladu s tim.

## Osnovna instalacija

[inline-code-attrs-start title = 'Instalacija widgeta za nedavne diskusije'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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

Funkcija `FastCommentsRecentDiscussionsV2` prihvata sledeće opcije konfiguracije:

- **tenantId** (obavezno): Vaš FastComments tenant ID
- **count** (opciono): Broj stranica za prikaz. Podrazumevano je `20`, maksimalno `100`
- **hasDarkBackground** (opciono): Forsira stil tamnog režima. Automatski se detektuje iz pozadine stranice ako nije postavljeno

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

### Forsiraj tamni režim

[inline-code-attrs-start title = 'Nedavne diskusije sa tamnim režimom'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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