`fastcomments-react` का हर विजेट उसी नाम के तहत उपलब्ध है:

| Component | Handle type | Embed loaded |
| --- | --- | --- |
| `FastCommentsCommentWidget` | `FastCommentsCommentWidgetHandle` | प्रमुख लाइव कमेंटिंग विजेट |
| `FastCommentsCommentCountWidget` | `FastCommentsCommentCountWidgetHandle` | इनलाइन कमेंट-काउंट बैज |
| `FastCommentsLiveChatWidget` | `FastCommentsLiveChatWidgetHandle` | स्ट्रीमिंग लाइव-चैट विजेट |
| `FastCommentsCollabChatWidget` | `FastCommentsCollabChatWidgetHandle` | टेक्स्ट-एंकर्ड सहयोगी चैट |
| `FastCommentsImageChatWidget` | `FastCommentsImageChatWidgetHandle` | क्षेत्र-आधारित इमेज कमेंट्स |
| `FastCommentsRecentCommentsWidget` | `FastCommentsRecentCommentsWidgetHandle` | हाल के कमेंट्स फ़ीड |
| `FastCommentsRecentDiscussionsWidget` | `FastCommentsRecentDiscussionsWidgetHandle` | हाल की चर्चाओं की फ़ीड |
| `FastCommentsReviewsSummaryWidget` | `FastCommentsReviewsSummaryWidgetHandle` | स्टार-रेटिंग सारांश |
| `FastCommentsTopPagesWidget` | `FastCommentsTopPagesWidgetHandle` | सर्वाधिक कमेंट किए गए पेजों का लीडरबोर्ड |
| `FastCommentsUserActivityFeedWidget` | `FastCommentsUserActivityFeedWidgetHandle` | प्रति-उपयोगकर्ता गतिविधि टाइमलाइन |

### मौजूदा एलिमेंट से जुड़ने वाले विजेट

`FastCommentsCollabChatWidget` और `FastCommentsImageChatWidget` कॉलर द्वारा दिए गए
एलिमेंट में माउंट होते हैं। एक `targetRef` एक्सेसर पास करें जो माउंट होने के बाद एलिमेंट लौटाए:

```tsx
import { FastCommentsImageChatWidget } from 'fastcomments-solidjs';

export default function ImageChat() {
  let img: HTMLImageElement | undefined;
  return (
    <>
      <img ref={img} src="/screenshot.png" alt="" />
      <FastCommentsImageChatWidget
        tenantId="demo"
        urlId="my-image"
        targetRef={() => img}
      />
    </>
  );
}
```

### क्षेत्र (Regions)

विजेट के ट्रैफ़िक को EU क्लस्टर से रूट करने के लिए `region="eu"` पास करें।