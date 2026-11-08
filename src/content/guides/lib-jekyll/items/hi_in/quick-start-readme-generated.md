अपनी tenant id एक बार `_config.yml` में सेट करें:

```yaml
fastcomments:
  tenant_id: demo
```

फिर एक टैग जोड़ें जहाँ भी आप विजेट चाहते हैं — किसी लेआउट, पोस्ट या पेज में:

```liquid
{% raw %}{% fastcomments %}{% endraw %}
```

बस इतना ही। `demo` को अपनी FastComments tenant id से बदलें (इसे
[Settings > API/SSO](https://fastcomments.com/auth/my-account/api) के अंतर्गत पाएं)।