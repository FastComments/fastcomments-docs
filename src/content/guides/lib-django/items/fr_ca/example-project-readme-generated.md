A runnable showcase lives in [`example/`](https://github.com/FastComments/fastcomments-django/blob/main/example) : une application à rail latéral + scène principale avec une page par widget et une **page de connexion référençant des utilisateurs de démonstration pré‑alimentés**.  
Connectez‑vous avec l’un d’eux et les widgets de commentaire et de chat en direct authentifient cette identité via **Secure SSO**. Depuis ce répertoire :

```bash
python manage.py migrate
# Utilisez votre propre locataire pour voir Secure SSO en action (un secret API l'active) :
FASTCOMMENTS_TENANT_ID=... FASTCOMMENTS_API_KEY=... python manage.py runserver
```

Sans secret API, il revient au locataire public `demo` (anonyme).  
[`example/browser_smoke.py`](https://github.com/FastComments/fastcomments-django/blob/main/example/browser_smoke.py) est un test e2e Playwright qui charge la page dans un vrai navigateur et publie un commentaire en tant qu’utilisateur Secure‑SSO.