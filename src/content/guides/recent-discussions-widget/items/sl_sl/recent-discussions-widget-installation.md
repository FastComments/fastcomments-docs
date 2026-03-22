Pripomoček Nedavne razprave prikazuje strani na vašem spletnem mestu, ki imajo najnovejšo aktivnost komentarjev. Vsak vnos prikaže naslov strani, datum zadnje aktivnosti in skupno število komentarjev. Samodejno zazna temna ozadja in temu primerno prilagodi svoj stil.

## Osnovna namestitev

[inline-code-attrs-start title = 'Namestitev pripomočka Nedavne razprave'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/widget-recent-discussions-v2.min.js"></script>
<div id="fastcomments-widget-recent-discussions"></div>
<script>
    FastCommentsRecentDiscussionsV2(document.getElementById('fastcomments-widget-recent-discussions'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

## Možnosti konfiguracije

Funkcija `FastCommentsRecentDiscussionsV2` sprejme naslednje možnosti konfiguracije:

- **tenantId** (required): Your FastComments tenant ID
- **count** (optional): Število strani za prikaz. Privzeto je `20`, največ `100`
- **hasDarkBackground** (optional): Prisilno omogoči temno oblikovanje. Če ni nastavljeno, se samodejno zazna glede na ozadje strani

## Napredni primeri

### Prilagojeno število

[inline-code-attrs-start title = 'Nedavne razprave s prilagojenim številom'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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

### Prisilni temni način

[inline-code-attrs-start title = 'Nedavne razprave s temnim načinom'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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