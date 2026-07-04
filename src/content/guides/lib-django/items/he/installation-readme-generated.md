התקן משורת שחרור (פרויקט זה מופץ באמצעות תגיות git, ולא דרך PyPI):

```bash
pip install "git+https://github.com/fastcomments/fastcomments-django.git@v0.1.0"
```

לגישה ל‑REST בצד השרת (העוזרים `admin()` / `public_api()`), הוסף את ה‑`api` הנוסף, שמביא את ה‑client שנוצר על‑ידי ה‑SDK:

```bash
pip install "fastcomments-django[api] @ git+https://github.com/fastcomments/fastcomments-django.git@v0.1.0"
```

הוסף את האפליקציה ל‑`INSTALLED_APPS`:

```python
INSTALLED_APPS = [
    # ...
    "fastcomments_django",
]
```