Um showcase executável está em [`example/`](https://github.com/FastComments/fastcomments-django/blob/main/example): um aplicativo de barra lateral esquerda + palco principal com uma página por widget e uma **página de login listando usuários de demonstração pré-carregados**. Faça login como qualquer um deles e os widgets de comentário e chat ao vivo autenticam essa identidade via **Secure SSO**. A partir desse diretório:

```bash
python manage.py migrate
# Use seu próprio tenant para ver o Secure SSO em ação (uma secret de API habilita isso):
FASTCOMMENTS_TENANT_ID=... FASTCOMMENTS_API_KEY=... python manage.py runserver
```

Sem um secret de API ele volta ao tenant público `demo` (anônimo). [`example/browser_smoke.py`](https://github.com/FastComments/fastcomments-django/blob/main/example/browser_smoke.py) é um teste e2e do Playwright que carrega a página em um navegador real e publica um comentário como o usuário Secure-SSO.