Κάθε widget από το `fastcomments-react` είναι διαθέσιμο με το ίδιο όνομα:

| Component | Τύπος handle | Ενσωματωμένο widget |
| --- | --- | --- |
| `FastCommentsCommentWidget` | `FastCommentsCommentWidgetHandle` | Κύριο widget ζωντανών σχολίων |
| `FastCommentsCommentCountWidget` | `FastCommentsCommentCountWidgetHandle` | Ενσωματωμένο badge μετρητή σχολίων |
| `FastCommentsLiveChatWidget` | `FastCommentsLiveChatWidgetHandle` | Widget live chat με ροή |
| `FastCommentsCollabChatWidget` | `FastCommentsCollabChatWidgetHandle` | Συνεργατικό chat με αγκύρωση σε κείμενο |
| `FastCommentsImageChatWidget` | `FastCommentsImageChatWidgetHandle` | Σχόλια εικόνας βάσει περιοχής |
| `FastCommentsRecentCommentsWidget` | `FastCommentsRecentCommentsWidgetHandle` | Ροή πρόσφατων σχολίων |
| `FastCommentsRecentDiscussionsWidget` | `FastCommentsRecentDiscussionsWidgetHandle` | Ροή πρόσφατων συζητήσεων |
| `FastCommentsReviewsSummaryWidget` | `FastCommentsReviewsSummaryWidgetHandle` | Σύνοψη βαθμολογιών με αστέρια |
| `FastCommentsTopPagesWidget` | `FastCommentsTopPagesWidgetHandle` | Κατάταξη σελίδων με τα περισσότερα σχόλια |
| `FastCommentsUserActivityFeedWidget` | `FastCommentsUserActivityFeedWidgetHandle` | Χρονολόγιο δραστηριότητας ανά χρήστη |

### Widgets που συνδέονται με υπάρχον στοιχείο

Τα `FastCommentsCollabChatWidget` και `FastCommentsImageChatWidget` τοποθετούνται σε στοιχείο που παρέχει ο καλών. Περάστε μια συνάρτηση `targetRef` που επιστρέφει το στοιχείο μόλις φορτώσει:

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

Περάστε `region="eu"` για να δρομολογείται η κίνηση του widget μέσω του ευρωπαϊκού (EU) cluster.