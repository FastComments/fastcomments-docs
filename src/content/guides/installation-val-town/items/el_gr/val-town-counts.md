---
Σε μια σελίδα ευρετηρίου, μην αποδίδετε ένα widget καταμέτρησης σχολίων ανά γραμμή. Αυτό σημαίνει ένα αίτημα ανά ανάρτηση. Χρησιμοποιήστε τη μαζική μέτρηση, η οποία απαιτεί ένα μόνο αίτημα για ολόκληρη τη σελίδα.

Σημειώστε κάθε γραμμή με το `urlId` που χρησιμοποιεί το νήμα της, και στη συνέχεια φορτώστε το μαζικό widget μία φορά:

[inline-code-attrs-start title = 'Μαζικές μετρήσεις σχολίων σε ευρετήριο'; type='javascript' inline-code-attrs-end]
[inline-code-start]
<ul>
  {posts.map((post) => (
    <li>
      <a href={post.slug}>{post.title}</a>{" "}
      <span class="fast-comments-count" data-fast-comments-url-id={post.slug}></span>
    </li>
  ))}
</ul>

<script
  dangerouslySetInnerHTML={{
    __html: `window.FastCommentsBulkCountConfig = ${
      JSON.stringify({ tenantId: TENANT_ID })
    };`,
  }}
/>
<script src="https://cdn.fastcomments.com/js/embed-widget-comment-count-bulk.min.js"></script>
[inline-code-end]

Το script εντοπίζει κάθε στοιχείο `.fast-comments-count` στη σελίδα και γεμίζει τη μέτρησή του.

`data-fast-comments-url-id` πρέπει να ταιριάζει με το `urlId` που χρησιμοποιεί το widget σχολίων της ανάρτησης. Εάν το widget χρησιμοποιεί το slug, ο δείκτης χρησιμοποιεί το slug. Μια ασυμφωνία εμφανίζει μηδέν σε ένα νήμα που έχει σχόλια.

Το script ελέγχει περιοδικά το `window.FastCommentsBulkCountConfig`, επομένως δεν έχει σημασία αν ορίσετε τη διαμόρφωση πριν ή μετά το στοιχείο script.
---