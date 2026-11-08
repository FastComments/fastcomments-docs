| घटक | विवरण |
| --- | --- |
| `FastComments` | उत्तर, वोटिंग और बहुत कुछ के साथ कमेंटिंग विजेट |
| `FastCommentsCommentCount` | किसी पेज के लिए कमेंट काउंट प्रदर्शित करता है |
| `FastCommentsImageChat` | इमेज एनोटेशन कमेंट्स |
| `FastCommentsLiveChat` | लाइव चैट विजेट |
| `FastCommentsCollabChat` | सहयोगात्मक इनलाइन कमेंटिंग |
| `FastCommentsReviewsSummary` | स्टार-रेटिंग समीक्षा सारांश |
| `FastCommentsUserActivityFeed` | उपयोगकर्ता गतिविधि फ़ीड |

सभी घटक पैकेज रूट से एक्सपोर्ट किए जाते हैं:

```astro
---
import { FastComments, FastCommentsLiveChat } from 'fastcomments-astro';
---
```