[`example/`](https://github.com/FastComments/fastcomments-django/tree/main/example) में एक चलाने योग्य डेमो मौजूद है: एक left-rail + main-stage
ऐप, जिसमें प्रत्येक विजेट के लिए एक पेज और **पहले से तैयार डेमो यूज़र्स की सूची दिखाने वाला साइन-इन पेज** है।
इनमें से किसी भी यूज़र के रूप में साइन इन करें और कमेंट तथा live-chat विजेट **Secure SSO** के माध्यम से उस
पहचान को प्रमाणित कर लेते हैं। उस डायरेक्टरी से:

```bash
python manage.py migrate
# Secure SSO को एक्शन में देखने के लिए अपना tenant इस्तेमाल करें (इसके लिए एक API secret चाहिए):
FASTCOMMENTS_TENANT_ID=... FASTCOMMENTS_API_KEY=... python manage.py runserver
```

API secret न होने पर यह सार्वजनिक `demo` tenant (anonymous) पर वापस आ जाता है।
[`example/browser_smoke.py`](https://github.com/FastComments/fastcomments-django/blob/main/example/browser_smoke.py) एक Playwright e2e है
जो असली ब्राउज़र में पेज लोड करता है और Secure-SSO
यूज़र के रूप में एक कमेंट पोस्ट करता है।