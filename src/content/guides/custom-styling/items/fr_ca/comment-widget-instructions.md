## Comment personnaliser le style du widget de commentaires

Vous pouvez personnaliser le style du widget de commentaires de deux façons :

### Option 1 : via le paramètre `customCSS`

Passez votre CSS personnalisé sous forme de chaîne au paramètre `customCSS` lors de l'initialisation du widget :

```javascript
window.fcConfigs = [{
    target: '#fastcomments-widget',
    tenantId: 'your-tenant-id',
    customCSS: `
        .fast-comments .comment {
            background-color: #f0f0f0 !important;
            border-radius: 8px !important;
        }
    `
}];
```

### Option 2 : via le tableau de bord d'administration

1. Allez à la [page de personnalisation du widget](https://fastcomments.com/auth/my-account/customize-widget) dans votre tableau de bord d'administration
2. Faites défiler jusqu'à la section "CSS personnalisé" sous "Avancé"
3. Saisissez votre CSS personnalisé
4. Cliquez sur "Enregistrer"

Votre CSS personnalisé sera appliqué à tous les widgets de commentaires de votre site.

## Conseils

- Utilisez `!important` pour outrepasser les styles par défaut si nécessaire
- Ciblez des sélecteurs spécifiques pour éviter d'affecter d'autres parties de votre site
- Testez votre CSS dans différents navigateurs pour la compatibilité
- Le widget utilise du CSS standard - aucun préprocesseur spécial requis