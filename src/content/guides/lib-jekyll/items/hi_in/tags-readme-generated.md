| टैग | विवरण |
| --- | --- |
| `fastcomments` | उत्तर, वोटिंग, मॉडरेशन और रीयलटाइम अपडेट के साथ लाइव कमेंटिंग |
| `fastcomments_comment_count` | वर्तमान पेज के लिए कमेंट संख्या |
| `fastcomments_comment_count_bulk` | एक सूची/इंडेक्स पेज पर कई पेजों के लिए कमेंट संख्या |
| `fastcomments_live_chat` | रीयलटाइम स्ट्रीमिंग चैट विजेट |
| `fastcomments_collab_chat` | सहयोगात्मक इनलाइन कमेंटिंग (टेक्स्ट एनोटेशन) |
| `fastcomments_image_chat` | इमेज एनोटेशन कमेंट्स |
| `fastcomments_recent_comments` | पूरी साइट के हाल के कमेंट्स |
| `fastcomments_recent_discussions` | हाल में सक्रिय चर्चा थ्रेड्स |
| `fastcomments_reviews_summary` | स्टार-रेटिंग समीक्षा सारांश |
| `fastcomments_top_pages` | सबसे अधिक चर्चित पेज |
| `fastcomments_user_activity_feed` | प्रति-उपयोगकर्ता गतिविधि फ़ीड |

### उदाहरण

```liquid
{% raw %}{# कमेंट संख्या। विजेट अपना लेबल स्वयं रेंडर करता है, जैसे "0 comments" #}
{% fastcomments_comment_count %}

{# लाइव चैट #}
{% fastcomments_live_chat %}

{# कोलैब चैट। CSS सेलेक्टर के साथ इसे एक कंटेंट एलिमेंट पर पॉइंट करें #}
<article id="post-body">
  <p>Highlight me to leave a comment.</p>
</article>
{% fastcomments_collab_chat target="#post-body" %}

{# इमेज चैट। CSS सेलेक्टर के साथ इसे एक इमेज एलिमेंट पर पॉइंट करें #}
<img id="hero" src="/hero.jpg" alt="Hero image">
{% fastcomments_image_chat target="#hero" %}

{# समीक्षा सारांश #}
{% fastcomments_reviews_summary %}

{# उपयोगकर्ता गतिविधि फ़ीड। user id आवश्यक है #}
{% fastcomments_user_activity_feed user_id="demo:demo-user" %}

{# ब्लॉग इंडेक्स के लिए बल्क कमेंट संख्या #}
{% for post in site.posts %}
  <a href="\{{ post.url }}">\{{ post.title }}</a>
  <span class="fast-comments-count" data-fast-comments-url-id="\{{ post.url }}"></span>
{% endfor %}
{% fastcomments_comment_count_bulk %}{% endraw %}
```