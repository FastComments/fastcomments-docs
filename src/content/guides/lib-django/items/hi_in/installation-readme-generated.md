रिलीज़ टैग से इंस्टॉल करें (यह प्रोजेक्ट git टैग के माध्यम से वितरित किया जाता है, PyPI से नहीं):

```bash
pip install "git+https://github.com/fastcomments/fastcomments-django.git@v0.1.0"
```

सर्वर-साइड REST एक्सेस के लिए (`admin()` / `public_api()` हेल्पर्स), `api` एक्स्ट्रा जोड़ें, जो SDK के जनरेटेड क्लाइंट को शामिल करता है:

```bash
pip install "fastcomments-django[api] @ git+https://github.com/fastcomments/fastcomments-django.git@v0.1.0"
```

ऐप को `INSTALLED_APPS` में जोड़ें:

```python
INSTALLED_APPS = [
    # ...
    "fastcomments_django",
]
```