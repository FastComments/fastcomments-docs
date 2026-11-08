| शॉर्टकोड | विवरण |
| --- | --- |
| `fastcomments` | उत्तर, वोटिंग और @mentions के साथ थ्रेडेड टिप्पणियाँ |
| `fastcomments-comment-count` | एक पेज के लिए टिप्पणी संख्या |
| `fastcomments-comment-count-bulk` | एक ही अनुरोध में कई पेजों के लिए टिप्पणी संख्याएँ ([बल्क टिप्पणी संख्याएँ](#bulk-comment-counts-readme-generated) देखें) |
| `fastcomments-live-chat` | लाइव चैट विजेट |
| `fastcomments-collab-chat` | सहयोगात्मक इनलाइन टिप्पणी (`target` आवश्यक है) |
| `fastcomments-image-chat` | छवि एनोटेशन टिप्पणियाँ (`target` आवश्यक है) |
| `fastcomments-recent-comments` | पूरी साइट में हाल की टिप्पणियाँ |
| `fastcomments-recent-discussions` | हाल में सक्रिय चर्चा थ्रेड |
| `fastcomments-reviews-summary` | स्टार-रेटिंग समीक्षा सारांश |
| `fastcomments-top-pages` | सबसे अधिक चर्चित पेज |
| `fastcomments-user-activity-feed` | प्रति-उपयोगकर्ता गतिविधि फ़ीड (`userId` आवश्यक है) |

### उदाहरण

टेक्स्ट के साथ इनलाइन टिप्पणी संख्या:

```text
इस पेज पर \{{< fastcomments-comment-count >}} टिप्पणियाँ हैं।
```

लाइव चैट:

```text
\{{< fastcomments-live-chat >}}
```

CSS सेलेक्टर द्वारा किसी कंटेंट एलिमेंट को लक्षित करते हुए कोलैब चैट:

```text
<article id="post-body">
  <p>टिप्पणी छोड़ने के लिए मुझे हाइलाइट करें।</p>
</article>

\{{< fastcomments-collab-chat target="#post-body" >}}
```

CSS सेलेक्टर द्वारा किसी छवि एलिमेंट को लक्षित करते हुए इमेज चैट:

```text
<img id="hero" src="/hero.jpg" alt="Hero image" />

\{{< fastcomments-image-chat target="#hero" >}}
```

समीक्षा सारांश:

```text
\{{< fastcomments-reviews-summary >}}
```

उपयोगकर्ता गतिविधि फ़ीड:

```text
\{{< fastcomments-user-activity-feed userId="demo:demo-user" >}}
```