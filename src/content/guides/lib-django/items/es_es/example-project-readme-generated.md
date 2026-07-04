Una demostración ejecutable se encuentra en [`example/`](https://github.com/FastComments/fastcomments-django/blob/main/example): una aplicación de carril izquierdo + escenario principal con una página por widget y una **página de inicio de sesión que enumera usuarios de demostración pre-semilla**.  
Inicia sesión con cualquiera de ellos y los widgets de comentarios y chat en vivo autentican esa identidad a través de **Secure SSO**. Desde ese directorio:

```bash
python manage.py migrate
# Use your own tenant to see Secure SSO in action (an API secret enables it):
FASTCOMMENTS_TENANT_ID=... FASTCOMMENTS_API_KEY=... python manage.py runserver
```

Sin una clave API vuelve al inquilino público `demo` (anónimo).  
[`example/browser_smoke.py`](https://github.com/FastComments/fastcomments-django/blob/main/example/browser_smoke.py) es una prueba e2e de Playwright que carga la página en un navegador real y publica un comentario como el usuario Secure-SSO.