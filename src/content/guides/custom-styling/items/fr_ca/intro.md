Ce guide fournit le CSS par défaut complet utilisé par le widget de commentaires FastComments (v2) et quelques instructions
pour surcharger les styles.

Comprendre le CSS par défaut vous permet de :

- **Personnaliser l'apparence** en surchargeant des styles spécifiques
- **Résoudre les problèmes de style** en voyant quelles classes et sélecteurs sont disponibles
- **Créer des thèmes personnalisés** compatibles avec la structure du widget
- **Utiliser avec des assistants d'IA** pour générer des modifications CSS personnalisées.

## Comment surcharger les styles

La façon de surcharger les styles varie selon le widget. Pour le widget de commentaires, vous devez utiliser le paramètre de configuration `customCSS` pour
transmettre le CSS dans l'iframe, ou spécifier le CSS dans la page de Personnalisation du tableau de bord administrateur qui servira le CSS
depuis notre CDN.