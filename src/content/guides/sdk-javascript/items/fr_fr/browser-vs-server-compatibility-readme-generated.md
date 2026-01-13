---
Ce SDK utilise **des points d'entrée doubles** pour garantir une compatibilité optimale et éviter les erreurs d'exécution :

- **`fastcomments-sdk/browser`** - Version sécurisée pour les navigateurs avec `fetch` natif
- **`fastcomments-sdk/server`** - Version complète pour Node.js avec prise en charge du SSO
- **`fastcomments-sdk`** (par défaut) - Contient uniquement des types, sûr à importer n'importe où
---