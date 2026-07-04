Εγκατάσταση από ετικέτα έκδοσης (το έργο αυτό διανέμεται μέσω ετικετών git, όχι μέσω PyPI):

```bash
pip install "git+https://github.com/fastcomments/fastcomments-django.git@v0.1.0"
```

Για πρόσβαση REST από την πλευρά του διακομιστή (τα βοηθητικά `admin()` / `public_api()`), προσθέστε το πρόσθετο `api`, το οποίο φέρνει τον δημιουργημένο πελάτη του SDK:

```bash
pip install "fastcomments-django[api] @ git+https://github.com/fastcomments/fastcomments-django.git@v0.1.0"
```

Προσθέστε την εφαρμογή στο `INSTALLED_APPS`:

```python
INSTALLED_APPS = [
    # ...
    "fastcomments_django",
]
```