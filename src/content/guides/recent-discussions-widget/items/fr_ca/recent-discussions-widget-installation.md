Le widget Discussions récentes affiche les pages de votre site ayant l'activité de commentaires la plus récente. Chaque élément affiche le titre de la page, la date de la dernière activité et le nombre total de commentaires. Il détecte automatiquement les arrière-plans sombres et ajuste son style en conséquence.

## Installation de base

[inline-code-attrs-start title = 'Installation du widget Discussions récentes'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/widget-recent-discussions-v2.min.js"></script>
<div id="fastcomments-widget-recent-discussions"></div>
<script>
    FastCommentsRecentDiscussionsV2(document.getElementById('fastcomments-widget-recent-discussions'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

## Options de configuration

La fonction `FastCommentsRecentDiscussionsV2` accepte les options de configuration suivantes :

- **tenantId** (requis) : Votre ID de locataire FastComments
- **count** (optionnel) : Nombre de pages à afficher. La valeur par défaut est `20`, maximum `100`
- **hasDarkBackground** (optionnel) : Forcer le style en mode sombre. Détecté automatiquement à partir de l'arrière-plan de la page si non configuré

## Exemples avancés

### Nombre personnalisé

[inline-code-attrs-start title = 'Discussions récentes avec nombre personnalisé'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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

### Forcer le mode sombre

[inline-code-attrs-start title = 'Discussions récentes avec mode sombre'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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