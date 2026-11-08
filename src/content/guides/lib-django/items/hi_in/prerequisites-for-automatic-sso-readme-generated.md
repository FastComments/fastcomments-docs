लॉग-इन किए गए उपयोगकर्ता को विजेट में स्वचालित रूप से पास करने के लिए, टैग रिक्वेस्ट से वर्तमान उपयोगकर्ता को पढ़ते हैं। सुनिश्चित करें कि आपके प्रोजेक्ट में ये दोनों मौजूद हैं (ये एक मानक Django प्रोजेक्ट में डिफ़ॉल्ट रूप से चालू रहते हैं):

- `TEMPLATES["OPTIONS"]["context_processors"]` में `django.template.context_processors.request`
- `MIDDLEWARE` में `django.contrib.auth.middleware.AuthenticationMiddleware`

टेम्पलेट कॉन्टेक्स्ट में रिक्वेस्ट न होने पर, विजेट एक अनाम (anonymous) विज़िटर के लिए रेंडर होते हैं। आप हमेशा उपयोगकर्ता को स्पष्ट रूप से पास कर सकते हैं: `{% fastcomments user=some_user %}`।