Une démonstration exécutable se trouve dans [`example/`](https://github.com/FastComments/fastcomments-django/blob/main/example) : une application à rail latéral + scène principale avec une page par widget et une **page de connexion répertoriant des utilisateurs de démonstration pré‑alimentés**.  
Connectez‑vous avec l’un d’eux et les widgets de commentaire et de chat en direct authentifient cette identité via **Secure SSO**. Depuis ce répertoire :

```bash
python manage.py migrate
# Use your own tenant to see Secure SSO in action (an API secret enables it):
FASTCOMMENTS_TENANT_ID=... FASTCOMMENTS_API_KEY=... python manage.py runserver
```

Sans secret d’API, cela revient au locataire public `demo` (anonyme).  
[`example/browser_smoke.py`](https://github.com/FastComments/fastcomments-django/blob/main/example/browser_smoke.py) est un test e2e Playwright qui charge la page dans un vrai navigateur et publie un commentaire en tant qu’utilisateur Secure‑SSO.