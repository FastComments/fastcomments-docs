The Recent Discussions Widget viser sider på dit websted, der har den mest nylige kommentaraktivitet. Hver post viser sidens titel, dato for seneste aktivitet og det samlede antal kommentarer. Den registrerer automatisk mørke baggrunde og tilpasser sit udseende derefter.

## Grundlæggende installation

[inline-code-attrs-start title = 'Installation af Recent Discussions-widget'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/widget-recent-discussions-v2.min.js"></script>
<div id="fastcomments-widget-recent-discussions"></div>
<script>
    FastCommentsRecentDiscussionsV2(document.getElementById('fastcomments-widget-recent-discussions'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

## Konfigurationsindstillinger

Funktionen `FastCommentsRecentDiscussionsV2` accepterer følgende konfigurationsmuligheder:

- **tenantId** (påkrævet): Din FastComments tenant-id
- **count** (valgfri): Antal sider der vises. Standard er `20`, maks `100`
- **hasDarkBackground** (valgfri): Tving mørk tilstand. Hvis ikke angivet, detekteres det automatisk ud fra sidebaggrunden

## Avancerede eksempler

### Brugerdefineret antal

[inline-code-attrs-start title = 'Recent Discussions med brugerdefineret antal'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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

### Tving mørk tilstand

[inline-code-attrs-start title = 'Recent Discussions med mørk tilstand'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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