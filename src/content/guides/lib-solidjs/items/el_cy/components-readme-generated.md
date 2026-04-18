Κάθε widget από το `fastcomments-react` είναι διαθέσιμο με το ίδιο όνομα:

| Component | Handle type | Embed loaded |
| --- | --- | --- |
| `FastCommentsCommentWidget` | `FastCommentsCommentWidgetHandle` | Κύριο widget για ζωντανά σχόλια |
| `FastCommentsCommentCountWidget` | `FastCommentsCommentCountWidgetHandle` | Ενσωματωμένη ένδειξη πλήθους σχολίων |
| `FastCommentsLiveChatWidget` | `FastCommentsLiveChatWidgetHandle` | Widget ζωντανής συνομιλίας με streaming |
| `FastCommentsCollabChatWidget` | `FastCommentsCollabChatWidgetHandle` | Συνεργατική συνομιλία αγκυρωμένη στο κείμενο |
| `FastCommentsImageChatWidget` | `FastCommentsImageChatWidgetHandle` | Σχόλια εικόνας βασισμένα σε περιοχές |
| `FastCommentsRecentCommentsWidget` | `FastCommentsRecentCommentsWidgetHandle` | Ροή πρόσφατων σχολίων |
| `FastCommentsRecentDiscussionsWidget` | `FastCommentsRecentDiscussionsWidgetHandle` | Ροή πρόσφατων συζητήσεων |
| `FastCommentsReviewsSummaryWidget` | `FastCommentsReviewsSummaryWidgetHandle` | Σύνοψη αξιολογήσεων με αστέρια |
| `FastCommentsTopPagesWidget` | `FastCommentsTopPagesWidgetHandle` | Κατάταξη σελίδων με τα περισσότερα σχόλια |
| `FastCommentsUserActivityFeedWidget` | `FastCommentsUserActivityFeedWidgetHandle` | Χρονολόγιο δραστηριότητας ανά χρήστη |

### Widgets που επισυνάπτονται σε υπάρχον στοιχείο

`FastCommentsCollabChatWidget` και `FastCommentsImageChatWidget` εγκαθίστανται μέσα σε στοιχείο που παρέχεται από τον καλούντα. Περάστε έναν accessor `targetRef` που επιστρέφει το στοιχείο αφού εγκατασταθεί:

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

### Περιοχές

Περάστε `region="eu"` για να δρομολογήσετε την κίνηση του widget μέσω του cluster της ΕΕ.