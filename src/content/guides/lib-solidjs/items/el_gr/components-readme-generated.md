Κάθε widget από `fastcomments-react` είναι διαθέσιμο με το ίδιο όνομα:

| Component | Handle type | Embed loaded |
| --- | --- | --- |
| `FastCommentsCommentWidget` | `FastCommentsCommentWidgetHandle` | Κύριο widget σχολιασμών σε πραγματικό χρόνο |
| `FastCommentsCommentCountWidget` | `FastCommentsCommentCountWidgetHandle` | Ενσωματωμένο badge καταμέτρησης σχολίων |
| `FastCommentsLiveChatWidget` | `FastCommentsLiveChatWidgetHandle` | Widget ζωντανής συνομιλίας streaming |
| `FastCommentsCollabChatWidget` | `FastCommentsCollabChatWidgetHandle` | Συνεργατική συνομιλία αγκυρωμένη σε κείμενο |
| `FastCommentsImageChatWidget` | `FastCommentsImageChatWidgetHandle` | Σχόλια εικόνας βασισμένα σε περιοχές |
| `FastCommentsRecentCommentsWidget` | `FastCommentsRecentCommentsWidgetHandle` | Ροή πρόσφατων σχολίων |
| `FastCommentsRecentDiscussionsWidget` | `FastCommentsRecentDiscussionsWidgetHandle` | Ροή πρόσφατων συζητήσεων |
| `FastCommentsReviewsSummaryWidget` | `FastCommentsReviewsSummaryWidgetHandle` | Σύνοψη αξιολόγησης με αστέρια |
| `FastCommentsTopPagesWidget` | `FastCommentsTopPagesWidgetHandle` | Κατάταξη σελίδων με τα περισσότερα σχόλια |
| `FastCommentsUserActivityFeedWidget` | `FastCommentsUserActivityFeedWidgetHandle` | Χρονική γραμμή δραστηριότητας ανά χρήστη |

### Widgets that attach to an existing element

`FastCommentsCollabChatWidget` and `FastCommentsImageChatWidget` mount into a caller-supplied
element. Pass a `targetRef` accessor that returns the element once mounted:

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

### Regions

Pass `region="eu"` to route widget traffic through the EU cluster.