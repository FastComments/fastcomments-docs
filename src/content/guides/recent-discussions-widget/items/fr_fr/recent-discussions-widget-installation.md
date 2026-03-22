Le widget Discussions récentes affiche les pages de votre site qui ont l'activité de commentaires la plus récente. Chaque entrée affiche le titre de la page, la date de la dernière activité et le nombre total de commentaires. Il détecte automatiquement les arrière-plans sombres et ajuste son style en conséquence.

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

- **tenantId** (required): Votre identifiant de locataire FastComments
- **count** (optional): Nombre de pages à afficher. La valeur par défaut est `20`, maximum `100`
- **hasDarkBackground** (optional): Forcer le style en mode sombre. Détecté automatiquement à partir de l'arrière-plan de la page si non configuré

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

[inline-code-attrs-start title = 'Discussions récentes en mode sombre'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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