A runnable showcase lives in [`example/`](https://github.com/FastComments/fastcomments-django/blob/main/example): bir sol-kenar + ana-sahne uygulaması; her widget için bir sayfa ve **önceden doldurulmuş demo kullanıcılarını listeleyen bir giriş sayfası**.

Bunlardan herhangi biriyle oturum açın ve yorum ve canlı‑chat widget’ları bu kimliği **Secure SSO** aracılığıyla doğrular.

Bu dizinden:

```bash
python manage.py migrate
# Secure SSO'yu eylemde görmek için kendi kiracınızı kullanın (bir API gizli anahtarı bunu etkinleştirir):
FASTCOMMENTS_TENANT_ID=... FASTCOMMENTS_API_KEY=... python manage.py runserver
```

Bir API gizli anahtarı olmadan, genel `demo` kiracısına geri döner (anonim).

[`example/browser_smoke.py`](https://github.com/FastComments/fastcomments-django/blob/main/example/browser_smoke.py) bir Playwright e2e’dir; gerçek bir tarayıcıda sayfayı yükler ve Secure‑SSO kullanıcısı olarak bir yorum gönderir.