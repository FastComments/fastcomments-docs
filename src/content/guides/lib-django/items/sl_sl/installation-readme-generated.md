Namestite iz oznake izdaj (ta projekt je distribuiran prek git oznak, ne preko PyPI):

```bash
pip install "git+https://github.com/fastcomments/fastcomments-django.git@v0.1.0"
```

Za strežniški REST dostop (pomožna orodja `admin()` / `public_api()`), dodajte
`api` dodatno komponento, ki naloži generiranega odjemalca SDK:

```bash
pip install "fastcomments-django[api] @ git+https://github.com/fastcomments/fastcomments-django.git@v0.1.0"
```

Dodajte aplikacijo v `INSTALLED_APPS`:

```python
INSTALLED_APPS = [
    # ...
    "fastcomments_django",
]
```